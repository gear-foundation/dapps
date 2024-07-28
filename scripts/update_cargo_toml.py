import re
import requests
from packaging import version

# GitHub API URL to get the latest tags
GEAR_REPO_TAGS_URL = "https://api.github.com/repos/gear-tech/gear/tags"

# ABSOLUTE PATH to cargo file
CARGO_FILE_PATH = '../contracts/Cargo.toml'

def get_latest_semver_tag():
    response = requests.get(GEAR_REPO_TAGS_URL)
    response.raise_for_status()
    tags = response.json()
    # Filter out tags that are not valid semantic versions
    valid_tags = [tag['name'] for tag in tags if re.match(r'^v?\d+\.\d+\.\d+$', tag['name'])]
    # Sort the valid tags by version
    valid_tags.sort(key=lambda s: version.parse(s.lstrip('v')), reverse=True)
    return valid_tags[0] if valid_tags else None

def update_cargo_toml(file_path, new_version):
    with open(file_path, 'r') as file:
        content = file.read()

    content = re.sub(r'gstd = ".*?"', f'gstd = "{new_version}"', content)
    content = re.sub(r'gear-wasm-builder = ".*?"', f'gear-wasm-builder = "{new_version}"', content)
    content = re.sub(r'gmeta = ".*?"', f'gmeta = "{new_version}"', content)
    content = re.sub(r'gclient = ".*?"', f'gclient = "{new_version}"', content)
    content = re.sub(r'gtest = { git = "https://github.com/gear-tech/gear", tag = ".*?" }', f'gtest = {{ git = "https://github.com/gear-tech/gear", tag = "v{new_version}" }}', content)
    content = re.sub(r'gear-core = ".*?"', f'gear-core = "{new_version}"', content)

    with open(file_path, 'w') as file:
        file.write(content)

def update_wf_contracts(file_path, new_version):
    with open(file_path, 'r') as file:
        content = file.read()

    content = re.sub(r'GEAR_VERSION: .*?', f'GEAR_VERSION: {new_version}', content)

    with open(file_path, 'w') as file:
        file.write(content)

if __name__ == "__main__":
    new_version = get_latest_semver_tag().lstrip('v')
    if new_version:
        update_cargo_toml('../contracts/Cargo.toml', new_version)
        update_wf_contracts('../.github/workflows/contracts-tests.yml', new_version)