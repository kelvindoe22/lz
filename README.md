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

Enjoy.  
[Go here if you want to here a badjoke.](https://kelvindoe22.github.io/hahanotfunny/)
