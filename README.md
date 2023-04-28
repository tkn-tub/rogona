# ROGONA -- Air-based Molecular Communication Simulator in a 2-Tx-1-Rx set-up

## What is Rogona?

This thesis work enhanced Rogona to simulate the air-based communication process between two spraying transmitters.  
It also added a reconstruction phase to decode the message and evaluate the bit error rate.

*Is it fast?* -- 1000 bits per Tx can be simulated and evaluated in under 4 seconds...

## Basic Usage

You will need to setup a Rust environment. You can use rustup or other methods as described in this [Rust Language Guide Installation Chapter](https://doc.rust-lang.org/stable/book/ch01-01-installation.html). After that the installation of Cargo should have been done implicitly.
Please check if cargo is working by
```bash
cargo --version
```
If you see a version number, you have it! If you see an error, such as "command not found", look at the documentation for your method of installation to determine how to install Cargo separately - as the [Hello, Cargo! Chapter](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html) states.

An air-based simulation can be run by:

```
cargo run --release Air {path\to\simulation_config.file} Threshold {path\to\reconstruction_config.file}
```

For the full Rogona experience you will need to create

1. a simulation configuration
2. a reconstruction configuration for learning
3. a learn configuration
4. a reconstruction configuration for applying

An example can be found in variants/study7  
Please keep in mind, that the input and output files should generally fit to each other -- But do not worry! Rust will tell you (*panickly*) if something went wrong.  
For learning reconstruction, the messages of the far and near should match each other so that the sequence can be specified.  
Sequence: 	h,f,n,l  
Msg_Far:	1100  
Msg_Near:	1010  

## Environment Variables

Rogona has many optional functions. Set  
- RUST_LOG="info" if you always want to know what is going on with your simulation. For a more in depth information you could also select "debug" and if your time is too valuable just set it to "warn".  
- BINARY_MSG="true", because... text was irrelevant for my studies but could be implemented in the future! The message you want to transmit is then only allowed to consist of the ascii characters 0 and 1  
- SIM2VID="true" if your storage space means nothing to you or you do not understand when molecules enter your receiver section. But on the positive side: You can generate a really cool video with that afterwards to woo your friends and family.
- PRINT_DEBUG="true" if you really want **all** the positions at every time step. Afterwards the [Blender Add-on](https://webgit.ccs-labs.org/git/CCS/pogona-blender) can hopefully help you to never having to do this again.
- SIM_ONLY="true" if you do not feel like reconverting your message (because... you designed it anyways.) or a reconstruction config is too confusing.
- RECON_ONLY="true" if you want this specific simulation data to be evaluated
- RECON_PRINT="true" with another command line argument with the path\to\where\cir\predictions\shall\be\stored
- RECON_FULL_PRINT="true" if not only the frames in the selection process shall be saved. BEWARE: reconstruction will then likely NOT choose the frame it normally would.
- CSV_PRINT="true" to store the results of the learn and apply phase somewhere
- APPEND="true" when you run 300 trials with 9 setups and 4 threshold-setting methods and do not want to check on the simulation 2 hours later to notice the csv file was overwritten every time. The csv file has to exist already when this is activated.
- FRAME_OFFSET="[int]" to put a constant offset to whichever sampling frame is calculated.

## Directory Structure

It makes sense to have a `sim`, `liv`, `recon`, `learn`, `apply`, `eval`, `far_out`, and `near_out` directory close to each other dedicated to the study you want to run. But it really does not matter! You are not bound to any of the names as you have to specify everything yourself in the configs and the path when you run the simulator.  
Just from experience... it makes sense.
Oh and all the paths you specify are relative!

## Doing Parameter Studies

You really want to analyze the impact of one of the parameters and have not done it, because making all the config files is tedious?
I got you covered.

```
cargo run --release addv {target\dir} {base_config\simulation} {variant_attr} {start} {step} {stop}
```

will copy your simulation base configuration except the parameter you specified and will replace it in a start-step-stop manner with the attribute. The attribute is also attached to the file-name and added to the LIV path plus `.txt`

```
cargo run --release addl {target\dir} {base_config\reconstruction} {base_config\learn} {variant_attr} {start} {step} {stop}
```

will copy your base reconstruction and learn configurations. Changing the attribute does not really do much except naming all your files including those inside the configuration accordingly. So you do not have to worry about the configs not matching each other. (Except you forgot to do it in the base_configs...)

```
cargo run --release adda {target\dir} {base_config\reconstruction} {bool} {variant_attr} {start} {step} {stop}
```
will do the same for your configurations in apply mode. The boolean asks whether your Learn Config has "Try" written as a threshold method as it produces 3 different apply files which all want to be accessed via a reconstruction config. If you input `true` Rogona will automatically hand you all 4.

And now you must be thinking: "here we go. typing cargo run --release a hundredth time", but i would like to stop you right there and recommend checking out the studyX.py files.
By just putting the correct names together and then running the shell commands en loop you can save a lot of time and sanity.

After you did one run of `cargo run --release` you can also use the produced `rogona_ab_molcom.exe` to run your simulations.

## Looking at the results

If you want to see colorful charts on how your simulation did why the bit error rate is far too high, check out visualizer.py.  
Unfortunately it is a rather long and crowded file, but that is the messy process of adapting continuously to different study setups.  
Some paths are hardcoded to the above mentioned directory structure, but apart from that just set it to study7, change the path and see what it does!

## Don't panic if...

- the src folder appears red, because `Serialize` and `Deserealize` are marked with this error: `proc macro Serialize not expanded: server is built without sysroot support`.
It is a bug by the rust-analyzer and compiling does work.
- you see thousands of warnings. Most of them are unused variables functioning as placeholder from a feature from old Pogona.

## If you are my examiner...

Hi! I hope you liked this more lighthearted approach after (or before) the academic thesis.  
My result figures are all stored in the directory study_results and every study in the base_config has an info.txt to inform you on the goal of the respective study.
The used scripts are the studyX.py files and the few python files in the sim2vid directory.

Hopefully I got everything covered.  

#### Thanks for reading!



