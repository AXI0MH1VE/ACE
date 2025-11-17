import yaml
def load_config(path="configs/directive.yaml"):
    with open(path, "r") as f:
        return yaml.safe_load(f)