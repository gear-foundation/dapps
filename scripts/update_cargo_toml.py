import re
import requests
from packaging import version

# GitHub API URL to get the latest tags
GEAR_REPO_TAGS_URL = "https://api.github.com/repos/gear-tech/gear/tags"

def get_latest_tag():
    response = requests.get(GEAR_REPO_TAGS_URL)
    response.raise_for_status()
    tags = response.json()
    if tags:
        latest_tag = tags[0]['name']
        return latest_tag
    return None

def update_cargo_toml(file_path, new_version):
    with open(file_path, 'r') as file:
        content = file.read()

    content = re.sub(r'gstd = ".*?"', f'gstd = "{new_version}"', content)
    content = re.sub(r'gear-wasm-builder = ".*?"', f'gear-wasm-builder = "{new_version}"', content)
    content = re.sub(r'gmeta = ".*?"', f'gmeta = "{new_version}"', content)
    content = re.sub(r'gclient = ".*?"', f'gclient = "{new_version}"', content)
    content = re.sub(r'gtest = { git = "https://github.com/gear-tech/gear", tag = ".*?" }', f'gtest = {{ git = "https://github.com/gear-tech/gear", tag = "{new_version}" }}', content)
    content = re.sub(r'gear-core = ".*?"', f'gear-core = "{new_version}"', content)

    with open(file_path, 'w') as file:
        file.write(content)

if __name__ == "__main__":
    new_version = get_latest_tag().lstrip('v')
    if new_version:
        update_cargo_toml('../contracts/Cargo.toml', new_version)
