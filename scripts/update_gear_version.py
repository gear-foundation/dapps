import re
import requests
from packaging import version

# GitHub API URLs to get the latest tags
GEAR_REPO_TAGS_URL = "https://api.github.com/repos/gear-tech/gear/tags"
SAILS_REPO_TAGS_URL = "https://api.github.com/repos/gear-tech/sails/tags"

# ABSOLUTE PATH to cargo file
CARGO_FILE_PATH = '../contracts/Cargo.toml'

def get_latest_gear_version(repo_url):
    response = requests.get(repo_url)
    response.raise_for_status()
    tags = response.json()
    # Filter out tags that are not valid semantic versions
    valid_tags = [tag['name'] for tag in tags if re.match(r'^v?\d+\.\d+\.\d+$', tag['name'])]
    # Sort the valid tags by version
    valid_tags.sort(key=lambda s: version.parse(s.lstrip('v')), reverse=True)
    return valid_tags[0] if valid_tags else None

def get_latest_sails_version(repo_url):
    response = requests.get(repo_url)
    response.raise_for_status()
    tags = response.json()
    # Filter out tags that are valid semantic versions with "rs/" prefix
    valid_tags = [tag['name'] for tag in tags if re.match(r'^rs/v?\d+\.\d+\.\d+$', tag['name'])]
    # Sort the valid tags by version
    valid_tags.sort(key=lambda s: version.parse(s.lstrip('rs/v')), reverse=True)
    return valid_tags[0] if valid_tags else None

def update_cargo_toml(file_path, gear_version, sails_version):
    with open(file_path, 'r') as file:
        content = file.read()

    # Log current version and updated version
    print(f"Updating Cargo.toml: Gear version {gear_version}, Sails version {sails_version}")

    # Update GEAR dependencies
    updated_content = content
    updated_content = re.sub(r'gstd = ".*?"', f'gstd = "{gear_version}"', updated_content)
    updated_content = re.sub(r'gear-wasm-builder = ".*?"', f'gear-wasm-builder = "{gear_version}"', updated_content)
    updated_content = re.sub(r'gmeta = ".*?"', f'gmeta = "{gear_version}"', updated_content)
    updated_content = re.sub(r'gclient = ".*?"', f'gclient = "{gear_version}"', updated_content)
    updated_content = re.sub(r'gtest = { git = "https://github.com/gear-tech/gear", tag = ".*?" }', f'gtest = {{ git = "https://github.com/gear-tech/gear", tag = "v{gear_version}" }}', updated_content)
    updated_content = re.sub(r'gear-core = ".*?"', f'gear-core = "{gear_version}"', updated_content)

    # Update SAILS dependencies (strip rs/v from version before writing)
    sails_version_clean = sails_version.lstrip('rs/v')  # Remove 'rs/' and 'v' from the version string
    updated_content = re.sub(r'sails-idl-gen = ".*?"', f'sails-idl-gen = "{sails_version_clean}"', updated_content)
    updated_content = re.sub(r'sails-rs = ".*?"', f'sails-rs = "{sails_version_clean}"', updated_content)
    updated_content = re.sub(r'sails-client-gen = ".*?"', f'sails-client-gen = "{sails_version_clean}"', updated_content)

    # If content has been updated, write it back to the file
    if content != updated_content:
        print(f"Changes detected. Writing updated Cargo.toml.")
        with open(file_path, 'w') as file:
            file.write(updated_content)
    else:
        print(f"No changes detected in Cargo.toml.")

def update_wf_contracts(file_path, gear_version):
    with open(file_path, 'r') as file:
        content = file.read()

    # Update GEAR version in workflow
    content = re.sub(r'GEAR_VERSION: .*', f'GEAR_VERSION: {gear_version}', content)

    with open(file_path, 'w') as file:
        file.write(content)

if __name__ == "__main__":
    # Get the latest GEAR version
    gear_version = get_latest_gear_version(GEAR_REPO_TAGS_URL).lstrip('v')
    print(f"Latest GEAR version: {gear_version}")
    
    # Get the latest SAILS version
    sails_version = get_latest_sails_version(SAILS_REPO_TAGS_URL).lstrip('rs/')
    print(f"Latest SAILS version: {sails_version}")
    
    if gear_version and sails_version:
        update_cargo_toml('../contracts/Cargo.toml', gear_version, sails_version)
        update_wf_contracts('../.github/workflows/contracts-tests.yml', gear_version)
    else:
        print("Could not find valid versions for GEAR or SAILS.")
