def build_path(prefix: str, basename: str, path: str) -> str:
    """
    Construct a new path, from arguments

    ```
    Example
        
        def build_path(a, b, c):
            ...

    Usage
        
        build_path(Prefix, BasePath, Path)
    ```
    """
    return "".join(f"{prefix}/{basename}/{path}")

