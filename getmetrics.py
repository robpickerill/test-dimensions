import boto3

client = boto3.client('cloudwatch')

response = client.get_metric_data(
    MetricDataQueries=[
        {
            'Id': 'e1',
            'MetricStat': {
                'Metric': {
                    'Namespace': 'AWS/Lambda',
                    'MetricName': 'ConcurrentExecutions',
                },
                'Period': 60,
                'Stat': 'Maximum',
            },
            'ReturnData': False,
        },
        {
            'Id': 'e2',
            'Expression': 'SERVICE_QUOTA(e1)',
            'ReturnData': True,
        }
    ],
    StartTime='2023-08-01T00:00:00Z',
    EndTime='2023-08-02T00:00:00Z',
)

print(response)