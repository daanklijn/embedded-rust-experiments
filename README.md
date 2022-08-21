# embedded-rust-experiments

In order to get a bit more of a feeling what rust is like on embedded devices, I did a couple of small experiments like blink, ..., etc.
For these experiments I used the stm32F401 plus a couple of elecrical components like LEDs, potentiometers and resistors. In order to make
the interaction with the hardware of this board a bit simpler, I make use of the stm32f4xx-hal crate, which simplifies a lot of this logic.

Each of the experiments can be found in their own dedicated file / folder. In order to run a specific one, make sure that it is being called
in `main.rs` and comment the other ones out.

After connecting to the stm32 using a StLink, the project can be run using `cargo build` and `cargo run`.
The `run` command will automaticaly fires up GDB and sets up the connection to OpenOCD.

Below you can find a short description of the experiments and the breadboard setup I used.

## blink

This experiment blinks a number of LED's one by one.

<img src="https://user-images.githubusercontent.com/27863547/185762635-9c2205dc-2eb5-4258-aac9-fd37eaa0d408.JPG" width="400px">


## adc_read

Reading the number of mV on pin 5 and printing it to the console.

## blink_w_speed_control

Same as the `blink` experiment, but now the speed at which the LED's blink can be adjusted using a potentiometer.

<img src="https://user-images.githubusercontent.com/27863547/185765522-341febbe-f6ad-4c7d-b6b7-da52c2500e26.JPG" width="400px">
