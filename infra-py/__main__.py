"""A Python Pulumi program"""

import pulumi
from pulumi_kubernetes.apps.v1 import Deployment
from pulumi_kubernetes.core.v1 import Service

appName = pulumi.Config().require("APP_NAME")
appLabels = {"app": appName}

_deployment = Deployment(
    f"{appName}-deployment",
    spec={
        "selector": {"match_labels": appLabels},
        "template": {
            "metadata": {"labels": appLabels},
            "spec": {
                "containers": [
                    {
                        "name": appName,
                        "image": f"{appName}:latest",
                        "image_pull_policy": "Never",
                        "ports": [{"container_port": 3000}],
                        "security_context": {"run_as_user": 10001},
                    }
                ],
            },
        },
    },
)

_service = Service(
    f"{appName}-service",
    metadata={"labels": appLabels},
    spec={
        "selector": appLabels,
        "ports": [{"port": 80, "target_port": 3000, "node_port": 30000}],
        "type": "NodePort",
    },
)
