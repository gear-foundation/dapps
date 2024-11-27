import re
import requests
from packaging import version

# GitHub API URLs to get the latest tags
GEAR_REPO_TAGS_URL = "https://api.github.com/repos/gear-tech/gear/tags"
SAILS_REPO_TAGS_URL = "https://api.github.com/repos/gear-tech/sails/tags"

# ABSOLUTE PATH to cargo file
CARGO_FILE_PATH = '../contracts/Cargo.toml'

def get_latest_semver_tag(repo_url, prefix=''):
    response = requests.get(repo_url)
    response.raise_for_status()
    tags = response.json()

    # Print the raw tags to debug
    print(f"Raw tags from {repo_url}: {tags}")
    
    # Build the regex pattern to account for the optional prefix (empty for gear, 'rs/' for sails)
    pattern = r'^' + re.escape(prefix) + r'\d+\.\d+\.\d+$'

    # Filter out tags that are not valid semantic versions
    valid_tags = [
        tag['name'] for tag in tags if re.match(pattern, tag['name'])
    ]
    
    if not valid_tags:
        print(f"No valid tags found in repository {repo_url}")
        return None
    
    # Remove prefix and sort by version
    valid_tags = [tag.lstrip(prefix) for tag in valid_tags]
    valid_tags.sort(key=lambda s: version.parse(s.lstrip('v')), reverse=True)

    return valid_tags[0] if valid_tags else None

def update_cargo_toml(file_path, gear_version, sails_version):
    with open(file_path, 'r') as file:
        content = file.read()

    # Update GEAR dependencies
    content = re.sub(r'gstd = ".*?"', f'gstd = "{gear_version}"', content)
    content = re.sub(r'gear-wasm-builder = ".*?"', f'gear-wasm-builder = "{gear_version}"', content)
    content = re.sub(r'gmeta = ".*?"', f'gmeta = "{gear_version}"', content)
    content = re.sub(r'gclient = ".*?"', f'gclient = "{gear_version}"', content)
    content = re.sub(r'gtest = { git = "https://github.com/gear-tech/gear", tag = ".*?" }', f'gtest = {{ git = "https://github.com/gear-tech/gear", tag = "v{gear_version}" }}', content)
    content = re.sub(r'gear-core = ".*?"', f'gear-core = "{gear_version}"', content)

    # Update SAILS dependencies
    content = re.sub(r'sails-idl-gen = ".*?"', f'sails-idl-gen = "{sails_version}"', content)
    content = re.sub(r'sails-rs = ".*?"', f'sails-rs = "{sails_version}"', content)
    content = re.sub(r'sails-client-gen = ".*?"', f'sails-client-gen = "{sails_version}"', content)

    with open(file_path, 'w') as file:
        file.write(content)

def update_wf_contracts(file_path, gear_version):
    with open(file_path, 'r') as file:
        content = file.read()

    # Update GEAR version in workflow
    content = re.sub(r'GEAR_VERSION: .*', f'GEAR_VERSION: {gear_version}', content)

    with open(file_path, 'w') as file:
        file.write(content)

if __name__ == "__main__":
    # Get the latest GEAR version (no 'rs/' prefix)
    gear_version = get_latest_semver_tag(GEAR_REPO_TAGS_URL).lstrip('v')
    
    # Get the latest SAILS version with 'rs/' prefix
    sails_version = get_latest_semver_tag(SAILS_REPO_TAGS_URL, prefix='rs/').lstrip('v')
    
    if gear_version and sails_version:
        update_cargo_toml('../contracts/Cargo.toml', gear_version, sails_version)
        update_wf_contracts('../.github/workflows/contracts-tests.yml', gear_version)
