


class Action:
    glb_flags: List[str]
    ext_flags: List[str]

    glb_vals: Dict[str, Any]
    ext_vals: Dict[str, Any]

    glb_directives: Dict[str, dict]
    ext_directives: Dict[str, dict]

    script: str
    default_return: str

