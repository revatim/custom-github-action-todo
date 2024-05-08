This is a project to learn about Docker, Github actions. 


Process Followed: 

#### Building Docker Image
```
docker build -t actions/left-todo .
```

### Define Github Workflow and Actions
Define action.yaml 
Define trigger as on workflow dispatch, checkout source code  
#### Testing github actions
Install act for testing github actions
```
brew install act
```
```
act -W .github/workflows/dev.yaml --container-architecture linux/amd64
```
