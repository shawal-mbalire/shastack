# Embedded: C++ (ESP32)

Hexagonal architecture for ESP32 using pure C++ (no Arduino.h in domain).

## Port (Abstract Class)

```cpp
class RelayPort {
public:
    virtual void turnOn() = 0;
    virtual void turnOff() = 0;
    virtual ~RelayPort() = default;
};
```

## Pure Domain Use Case

```cpp
class PumpController {
    RelayPort& pump;
    bool isActive = false;
public:
    PumpController(RelayPort& pump) : pump(pump) {}

    void toggleIrrigation() {
        if (isActive) {
            pump.turnOff();
        } else {
            pump.turnOn();
        }
        isActive = !isActive;
    }
};
```

## Driven Adapter (Hardware Specific)

```cpp
#include <Arduino.h>

class ESP32RelayAdapter : public RelayPort {
    uint8_t pin;
public:
    ESP32RelayAdapter(uint8_t pin) : pin(pin) {
        pinMode(pin, OUTPUT);
    }

    void turnOn() override { digitalWrite(pin, HIGH); }
    void turnOff() override { digitalWrite(pin, LOW); }
};
```

## Driving Adapter (Hardware Interrupt)

```cpp
ESP32RelayAdapter waterPump(14);
PumpController controller(waterPump);

void IRAM_ATTR buttonISR() {
    controller.toggleIrrigation();
}

void setup() {
    pinMode(0, INPUT_PULLUP);
    attachInterrupt(digitalPinToInterrupt(0), buttonISR, FALLING);
}

void loop() {
    // Main loop
}
```

## Cross-Cutting Concerns

### Logging

```cpp
// Port (Domain)
class Logger {
public:
    virtual void info(const char* msg) = 0;
    virtual void error(const char* msg) = 0;
    virtual ~Logger() = default;
};

// Adapter (Serial)
class SerialLogger : public Logger {
public:
    void info(const char* msg) override { Serial.println("[INFO] " + String(msg)); }
    void error(const char* msg) override { Serial.println("[ERROR] " + String(msg)); }
};

// Adapter (SD Card)
class SDLogger : public Logger {
    File logFile;
public:
    SDLogger(const char* path) { logFile = SD.open(path, FILE_WRITE); }
    void info(const char* msg) override { logFile.println("[INFO] " + String(msg)); }
    void error(const char* msg) override { logFile.println("[ERROR] " + String(msg)); }
};
```

### Configuration / Secrets

```cpp
// Adapter resolves config, domain receives as arguments
#include <Preferences.h>

struct Config {
    int interval;
    int maxRetries;
};

Config loadConfig() {
    Preferences prefs;
    prefs.begin("config");
    Config cfg;
    cfg.interval = prefs.getInt("interval", 5);
    cfg.maxRetries = prefs.getInt("max_retries", 3);
    prefs.end();
    return cfg;
}

// Domain receives as params
PumpController controller(waterPump, cfg.interval);
```

### Caching (Simple Buffer)

```cpp
template<typename T>
class CachedSensorAdapter {
    SensorPort& sensor;
    T cache;
    unsigned long lastRead;
    unsigned long cacheDuration;
public:
    CachedSensorAdapter(SensorPort& sensor, unsigned long duration)
        : sensor(sensor), lastRead(0), cacheDuration(duration) {}

    T read() {
        unsigned long now = millis();
        if ((now - lastRead) < cacheDuration) return cache;
        cache = sensor.read();
        lastRead = now;
        return cache;
    }
};
```

### Auth (Hardware-based)

```cpp
// Domain enforces rules
void togglePump(int userId, PumpController& controller, const int* allowedUsers, int count) {
    bool authorized = false;
    for (int i = 0; i < count; i++) {
        if (allowedUsers[i] == userId) { authorized = true; break; }
    }
    if (!authorized) {
        throw std::runtime_error("Unauthorized user");
    }
    controller.toggleIrrigation();
}
```

### Telemetry & Metrics

```cpp
// Port (Domain)
class Metrics {
public:
    virtual void increment(const char* name) = 0;
    virtual void gauge(const char* name, float value) = 0;
    virtual ~Metrics() = default;
};

// Adapter (MQTT)
#include <PubSubClient.h>

class MQTTMetrics : public Metrics {
    PubSubClient& client;
    String topicPrefix;
public:
    MQTTMetrics(PubSubClient& client, const char* prefix)
        : client(client), topicPrefix(prefix) {}

    void increment(const char* name) override {
        String topic = topicPrefix + "/" + name;
        client.publish(topic.c_str(), "1");
    }
    void gauge(const char* name, float value) override {
        String topic = topicPrefix + "/" + name;
        client.publish(topic.c_str(), String(value).c_str());
    }
};
```

### Event Publishing

```cpp
// Port (Domain)
class EventPublisher {
public:
    virtual void publish(const char* event, const char* data) = 0;
    virtual ~EventPublisher() = default;
};

// Adapter (MQTT)
class MQTTEventPublisher : public EventPublisher {
    PubSubClient& client;
    String topic;
public:
    MQTTEventPublisher(PubSubClient& client, const char* topic)
        : client(client), topic(topic) {}

    void publish(const char* event, const char* data) override {
        String payload = "{\"event\":\"" + String(event) + "\",\"data\":" + String(data) + "}";
        client.publish(topic.c_str(), payload.c_str());
    }
};
```

## Localized Concerns (ESP32 IoT)

### WiFi Management

```cpp
// Adapter (WiFi)
#include <WiFi.h>

class WiFiAdapter {
    String ssid;
    String password;
public:
    WiFiAdapter(const char* ssid, const char* password)
        : ssid(ssid), password(password) {}

    bool connect(int timeoutMs = 10000) {
        WiFi.begin(ssid.c_str(), password.c_str());
        unsigned long start = millis();
        while (WiFi.status() != WL_CONNECTED) {
            if (millis() - start > timeoutMs) return false;
            delay(500);
        }
        return true;
    }

    void disconnect() { WiFi.disconnect(); }
    bool isConnected() { return WiFi.status() == WL_CONNECTED; }
    String getIP() { return WiFi.localIP().toString(); }
};
```

### MQTT Client

```cpp
// Adapter (MQTT)
#include <PubSubClient.h>
#include <WiFiClient.h>

class MQTTClientAdapter {
    WiFiClient wifiClient;
    PubSubClient client;
public:
    MQTTClientAdapter(const char* broker, int port = 1883)
        : client(wifiClient) {
        client.setServer(broker, port);
    }

    bool connect(const char* clientId) {
        return client.connect(clientId);
    }

    void publish(const char* topic, const char* msg) {
        client.publish(topic, msg);
    }

    void subscribe(const char* topic, void (*callback)(char*, uint8_t*, unsigned int)) {
        client.setCallback(callback);
        client.subscribe(topic);
    }

    void loop() { client.loop(); }
    bool isConnected() { return client.connected(); }
};
```

### OTA Updates

```cpp
// Adapter (OTA via HTTP)
#include <HTTPClient.h>
#include <Update.h>

class OTAAdapter {
    String firmwareUrl;
public:
    OTAAdapter(const char* url) : firmwareUrl(url) {}

    bool checkAndUpdate() {
        HTTPClient http;
        http.begin(firmwareUrl);
        int code = http.GET();

        if (code == 200) {
            int contentLength = http.getSize();
            WiFiClient* stream = http.getStreamPtr();

            if (Update.begin(contentLength)) {
                Update.writeStream(*stream);
                if (Update.end()) {
                    ESP.restart();
                    return true;
                }
            }
        }
        http.end();
        return false;
    }
};
```

### Sleep Modes (Power Management)

```cpp
// Adapter (Power Management)
#include <esp_sleep.h>

class PowerManager {
public:
    void deepSleep(uint64_t microseconds) {
        esp_sleep_enable_timer_wakeup(microseconds);
        esp_deep_sleep_start();
    }

    void lightSleep(uint64_t microseconds) {
        esp_sleep_enable_timer_wakeup(microseconds);
        esp_light_sleep_start();
    }

    void enableWakeupOnPin(int pin, int level) {
        esp_sleep_enable_gpio_wakeup();
        gpio_wakeup_enable((gpio_num_t)pin,
            level ? GPIO_INTR_HIGH_LEVEL : GPIO_INTR_LOW_LEVEL);
    }

    const char* getWakeReason() {
        switch (esp_sleep_get_wakeup_cause()) {
            case ESP_SLEEP_WAKEUP_TIMER: return "timer";
            case ESP_SLEEP_WAKEUP_GPIO: return "gpio";
            case ESP_SLEEP_WAKEUP_ULP: return "ulp";
            default: return "other";
        }
    }
};
```

### ADC (Analog Sensors)

```cpp
// Adapter (ADC)
#include <driver/adc.h>

class ADCAdapter {
    adc1_channel_t channel;
public:
    ADCAdapter(adc_channel_t channel) : channel(channel) {
        adc1_config_width(ADC_WIDTH_BIT_12);
        adc1_config_channel_atten(channel, ADC_ATTEN_DB_11);
    }

    float readVoltage() {
        int raw = adc1_get_raw(channel);
        return raw * 3.3 / 4095.0;
    }

    float readMapped(float minVal, float maxVal) {
        int raw = adc1_get_raw(channel);
        return minVal + (raw / 4095.0) * (maxVal - minVal);
    }
};
```

### PWM (Motor/LED Control)

```cpp
// Adapter (PWM)
#include <driver/ledc.h>

class PWMAdapter {
    ledc_channel_t channel;
    ledc_timer_bit_t dutyResolution = LEDC_TIMER_10_BIT;
public:
    PWMAdapter(int pin, int freq = 1000) {
        ledc_timer_config_t timer_conf = {
            .speed_mode = LEDC_HIGH_SPEED_MODE,
            .duty_resolution = dutyResolution,
            .timer_num = LEDC_TIMER_0,
            .freq_hz = (uint32_t)freq,
            .clk_cfg = LEDC_AUTO_CLK
        };
        ledc_timer_config(&timer_conf);

        channel = LEDC_CHANNEL_0;
        ledc_channel_config_t channel_conf = {
            .gpio_num = pin,
            .speed_mode = LEDC_HIGH_SPEED_MODE,
            .channel = channel,
            .timer_sel = LEDC_TIMER_0,
            .duty = 0,
            .hpoint = 0
        };
        ledc_channel_config(&channel_conf);
    }

    void setDutyPercent(int percent) {
        int duty = (percent * 1023) / 100;
        ledc_set_duty(LEDC_HIGH_SPEED_MODE, channel, duty);
        ledc_update_duty(LEDC_HIGH_SPEED_MODE, channel);
    }

    void off() { setDutyPercent(0); }
};
```
