import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MODEL = ROOT / "examples" / "src" / "training" / "25. Transitions" / "Local Clock Example.sysml"
SCENARIO = ROOT / "examples" / "state_machine_scenario.json"


def main() -> None:
    command = [
        "cargo",
        "run",
        "-q",
        "-p",
        "mercurio-cli",
        "--",
        "reason",
        "state-machine-run",
        "--file",
        str(MODEL),
        "--machine",
        "ServerBehavior",
        "--scenario",
        str(SCENARIO),
        "--format",
        "json",
    ]
    result = subprocess.run(command, cwd=ROOT, check=True, capture_output=True, text=True)
    report = json.loads(result.stdout)
    print(f"{report['machine_id']} -> {report['status']}")
    for step in report["steps"]:
        transition = step["transition_id"] or "<blocked>"
        print(f"{step['step']}: {step['trigger']} via {transition}")
    print("active:", " > ".join(report["active_configuration"]))


if __name__ == "__main__":
    main()
