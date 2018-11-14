# gotta
This program is making image and animation for my Rust's practice. I reference to https://qiita.com/scivola/items/98c1f7284469c891cb50 .
My program has a defference. It is that you can specify some arguments in CLI.

# Usage
    gotta dir k1 k2 g t w h

* all argument without dir is integer of 16bit
* dir is target-directory for generating images. If path of dir doesn't exist, this program occured error.
* k1, k2, g are parameters for simulation
* w is width of image
* h is height of image
* t is time of simulation

When you run this program, it generates images such as img-0010 in "dir". So, if you want to make GIF or APNG, you can run other program.
