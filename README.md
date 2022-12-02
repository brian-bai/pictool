# Picture preprocessor tool for meshphormer algorithm training.

```
Usage: pictool [OPTIONS] --source <SOURCE> --target <TARGET> <COMMAND>

Commands:
  resize  
  flip    
  rotate  
  help    Print this message or the help of the given subcommand(s)

Options:
  -s, --source <SOURCE>  File path of the source pictures
  -t, --target <TARGET>  File path of the target pictures
  -p, --psize <PSIZE>    Target picture size, current requirement of meshphormer is 224x224 [default: 224]
  -h, --help             Print help information
  -V, --version          Print version information
```