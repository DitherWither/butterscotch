import subprocess


def system(cmd: str):
    output = ""
    try:
        output = subprocess.check_output(cmd, shell=True)
    except subprocess.CalledProcessError:
        print(output)
        print(f"Failed to run `{cmd}`")
        exit(-1)