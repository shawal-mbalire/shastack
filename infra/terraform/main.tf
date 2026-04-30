resource "null_resource" "placeholder" {
  provisioner "local-exec" {
    command = "echo shastack infrastructure deployed"
  }
}
