import * as k8s from '@pulumi/kubernetes'
import * as pulumi from '@pulumi/pulumi'

const appName = new pulumi.Config().require('APP_NAME')
const chartPath = new pulumi.Config().require('CHART_PATH')
// const appLabels = { app: appName }

const _chart = new k8s.helm.v3.Chart(appName, {
	path: chartPath,
	values: {
		image: {
			repository: appName,
			tag: 'latest',
			pullPolicy: 'Never',
		}
	}
})

// const _deployment = new k8s.apps.v1.Deployment(`${appName}-deployment`, {
// 	spec: {
// 		selector: { matchLabels: appLabels },
// 		template: {
// 			metadata: { labels: appLabels },
// 			spec: {
// 				containers: [
// 					{
// 						name: appName,
// 						image: `${appName}:latest`,
// 						imagePullPolicy: 'Never',
// 						ports: [{ containerPort: 3000 }],
// 						securityContext: {
// 							runAsUser: 10001,
// 						},
// 					},
// 				],
// 			},
// 		},
// 	},
// })

// const _service = new k8s.core.v1.Service(`${appName}-service`, {
// 	metadata: { labels: appLabels },
// 	spec: {
// 		selector: appLabels,
// 		ports: [{ port: 80, targetPort: 3000, nodePort: 30000 }],
// 		type: 'NodePort',
// 	},
// })
