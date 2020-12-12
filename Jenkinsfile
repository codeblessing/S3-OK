pipeline {
  agent any
  stages {
    stage('Build') {
      parallel {
        stage('Build') {
          steps {
            bat 'cargo build'
          }
        }

        stage('') {
          steps {
            bat 'cargo doc'
          }
        }

      }
    }

    stage('Test') {
      steps {
        bat 'cargo test'
      }
    }

  }
}