# LZ

Sometimes i just wasn't sure if a directory existed. So i had to ls and find it.  
But now with *LZ* you can try finding a directory or file without generating any errors

## How to use

Obviously you need to have Rust and Cargo installed.  
Download the repo with git clone or any other method.
Navigate into the lz folder  
Execute the following code in your shell

```powershell
cargo build --release
```

Now navigate to the target/release folder and copy the directory  
Add the directory to your path variable  
*It returns a dot when it can't find a file or directory with the name provided*

```
PS C:\> lz users
users
PS C:\> lz epstein
.
PS C:\> cd (lz users)
PS C:\users> cd ..
PS C:\> cd (lz epstein)
PS C:\>
```

Now supports other directories tooo
```
PS C:\Users\PC> lz c:/users
c:/users
PS C:\Users\PC> cd (lz c:/users)
PS C:\users> lz c:/hackers
.
PS C:\users> cd (lz c:/hackers)
PS C:\users>
```
  
## v3  
Now you can execute command when the directory or file is found
```
PS C:\Users\PC\documents\rust_projects\lz> lz src/main.rs -e code
```

This command will open vs code if its installed and the destination is found.


## v4
Now you can search for files that start with, end with or contain certain characters
or a character.
```
PS C:\Users\PC\documents\rust_projects\lz> lz src/m -sw
```
This command will search for files/folders in the src folder that start with m and lists them.    
Returns a dot if no folders are found.
```
PS C:\Users\PC\documents\rust_projects\lz> lz src/m -ew
```
This command will search for files/folders in the src folder that end with m and lists them.  
Returns a dot if no folders are found.
```
PS C:\Users\PC\documents\rust_projects\lz> lz src/m -c
```
This command will search for files/folders in the src folder that contain m and lists them.  
Returns a dot if no folders are found.






Enjoy.  
[Go here if you want to hear a badjoke.](https://kelvindoe22.github.io/hahanotfunny/)
