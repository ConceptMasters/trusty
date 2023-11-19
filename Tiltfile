docker_build('trusty.dev', '.', ignore=['target', 'tests'],
    dockerfile='./Dockerfile.dev',
    live_update=[
        sync('.', '/app'),
])
docker_compose('./docker-compose.yml')
