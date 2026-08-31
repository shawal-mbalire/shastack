# Embedded: MicroPython (ESP32)

Hexagonal architecture for ESP32 using MicroPython. Pure domain testable on PC without hardware.

## Port (Protocol / Duck Typing)

```python
class RelayPort:
    def turn_on(self) -> None: pass
    def turn_off(self) -> None: pass
```

## Pure Domain Use Case

```python
class PumpController:
    def __init__(self, pump_relay: RelayPort):
        self.pump = pump_relay
        self.is_active = False

    def toggle_irrigation(self):
        if self.is_active:
            self.pump.turn_off()
        else:
            self.pump.turn_on()
        self.is_active = not self.is_active
```

## Driven Adapter (Hardware Specific)

```python
from machine import Pin

class ESP32RelayAdapter(RelayPort):
    def __init__(self, pin_number: int):
        self.pin = Pin(pin_number, Pin.OUT)

    def turn_on(self) -> None:
        self.pin.value(1)

    def turn_off(self) -> None:
        self.pin.value(0)
```

## Driving Adapter (Hardware Interrupt)

```python
# Composition Root
water_pump = ESP32RelayAdapter(pin_number=14)
controller = PumpController(pump_relay=water_pump)

# Hardware driving the use case
button = Pin(0, Pin.IN, Pin.PULL_UP)
button.irq(trigger=Pin.IRQ_FALLING, handler=lambda p: controller.toggle_irrigation())
```

## Cross-Cutting Concerns

### Logging

```python
# Port (Domain)
class Logger:
    def info(self, msg: str) -> None: pass
    def error(self, msg: str) -> None: pass

# Adapter (Serial/UART)
from machine import UART

class SerialLogger(Logger):
    def __init__(self, uart_id=0, baudrate=115200):
        self.uart = UART(uart_id, baudrate=baudrate)
    def info(self, msg: str) -> None:
        self.uart.write(f"[INFO] {msg}\n")
    def error(self, msg: str) -> None:
        self.uart.write(f"[ERROR] {msg}\n")

# Adapter (File)
class FileLogger(Logger):
    def __init__(self, path="/logs/system.log"):
        self.path = path
    def info(self, msg: str) -> None:
        with open(self.path, "a") as f: f.write(f"[INFO] {msg}\n")
    def error(self, msg: str) -> None:
        with open(self.path, "a") as f: f.write(f"[ERROR] {msg}\n")
```

### Configuration / Secrets

```python
# Adapter resolves config, domain receives as arguments
import json

def load_config(path="/config.json"):
    with open(path) as f:
        return json.load(f)

# Domain receives as params
config = load_config()
controller = PumpController(pump_relay=water_pump, interval=config["interval"])
```

### Caching (Simple Buffer)

```python
class CachedSensorAdapter:
    def __init__(self, sensor: SensorPort, cache_duration=5):
        self.sensor = sensor
        self.cache = None
        self.last_read = 0
        self.cache_duration = cache_duration

    def read(self) -> float:
        import time
        now = time.time()
        if self.cache is not None and (now - self.last_read) < self.cache_duration:
            return self.cache
        self.cache = self.sensor.read()
        self.last_read = now
        return self.cache
```

### Auth (Hardware-based)

```python
# Domain enforces rules
def toggle_pump(user_id: int, controller: PumpController, allowed_users: list):
    if user_id not in allowed_users:
        raise PermissionError("Unauthorized user")
    controller.toggle_irrigation()
```

### Telemetry & Metrics

```python
# Port (Domain)
class Metrics:
    def increment(self, name: str) -> None: pass
    def gauge(self, name: str, value: float) -> None: pass

# Adapter (MQTT)
import umqtt.simple as mqtt

class MQTTMetrics(Metrics):
    def __init__(self, broker: str, topic_prefix: str):
        self.client = mqtt.MQTTClient("esp32", broker)
        self.topic_prefix = topic_prefix
    def increment(self, name: str) -> None:
        self.client.publish(f"{self.topic_prefix}/{name}", "1")
    def gauge(self, name: str, value: float) -> None:
        self.client.publish(f"{self.topic_prefix}/{name}", str(value))
```

### Event Publishing

```python
# Port (Domain)
class EventPublisher:
    def publish(self, event: str, data: dict) -> None: pass

# Adapter (MQTT)
class MQTTEventPublisher(EventPublisher):
    def __init__(self, broker: str, topic: str):
        self.client = mqtt.MQTTClient("esp32", broker)
        self.topic = topic
    def publish(self, event: str, data: dict) -> None:
        import json
        self.client.publish(self.topic, json.dumps({"event": event, "data": data}))
```

## Localized Concerns (ESP32 IoT)

### WiFi Management

```python
# Adapter (WiFi)
import network
import time

class WiFiAdapter:
    def __init__(self, ssid: str, password: str):
        self.sta = network.WLAN(network.STA_IF)
        self.ssid = ssid
        self.password = password

    def connect(self, timeout=10):
        self.sta.active(True)
        if not self.sta.isconnected():
            self.sta.connect(self.ssid, self.password)
            start = time.time()
            while not self.sta.isconnected():
                if time.time() - start > timeout:
                    raise TimeoutError("WiFi connection failed")
                time.sleep(0.5)
        print(f"Connected: {self.sta.ifconfig()[0]}")

    def disconnect(self):
        self.sta.disconnect()
        self.sta.active(False)

    def is_connected(self) -> bool:
        return self.sta.isconnected()
```

### MQTT Client

```python
# Adapter (MQTT)
import umqtt.simple as mqtt

class MQTTClientAdapter:
    def __init__(self, client_id: str, broker: str, port=1883):
        self.client = mqtt.MQTTClient(client_id, broker, port)

    def connect(self):
        self.client.connect()

    def publish(self, topic: str, msg: str):
        self.client.publish(topic, msg)

    def subscribe(self, topic: str, callback):
        self.client.set_callback(callback)
        self.client.subscribe(topic)

    def check_messages(self):
        self.client.check_msg()
```

### OTA Updates

```python
# Adapter (OTA via HTTP)
import urequests
import machine

class OTAAdapter:
    def __init__(self, firmware_url: str):
        self.firmware_url = firmware_url

    def check_and_update(self):
        try:
            response = urequests.get(self.firmware_url)
            if response.status_code == 200:
                # Save new firmware
                with open("/new_firmware.bin", "wb") as f:
                    f.write(response.content)
                # Reset to apply
                machine.reset()
        except Exception as e:
            print(f"OTA failed: {e}")
```

### Sleep Modes (Power Management)

```python
# Adapter (Power Management)
import machine
import esp32

class PowerManager:
    def __init__(self):
        self.light_sleep_enabled = False

    def deep_sleep(self, microseconds: int):
        machine.deepsleep(microseconds)

    def light_sleep(self, microseconds: int):
        machine.lightsleep(microseconds)

    def enable_wake_on_pin(self, pin_number: int, level: int):
        pin = machine.Pin(pin_number, machine.Pin.IN)
        esp32.wake_on_gpio(level)

    def get_wake_reason(self) -> str:
        reason = machine.reset_cause()
        if reason == machine.DEEPSLEEP_RESET:
            return "deep_sleep"
        elif reason == machine.SOFT_RESET:
            return "soft_reset"
        return "power_on"
```

### ADC (Analog Sensors)

```python
# Adapter (ADC)
from machine import ADC, Pin

class ADCAdapter:
    def __init__(self, pin_number: int, atten=ADC.ATTN_11DB):
        self.adc = ADC(Pin(pin_number))
        self.adc.atten(atten)

    def read_voltage(self) -> float:
        raw = self.adc.read()
        return raw * 3.3 / 4095  # ESP32 12-bit ADC

    def read_mapped(self, min_val: float, max_val: float) -> float:
        raw = self.adc.read()
        return min_val + (raw / 4095) * (max_val - min_val)
```

### PWM (Motor/LED Control)

```python
# Adapter (PWM)
from machine import Pin, PWM

class PWMAdapter:
    def __init__(self, pin_number: int, freq=1000):
        self.pwm = PWM(Pin(pin_number))
        self.pwm.freq(freq)

    def set_duty(self, duty_percent: int):
        self.pwm.duty(int(duty_percent * 1023 / 100))

    def off(self):
        self.pwm.duty(0)

    def deinit(self):
        self.pwm.deinit()
```
