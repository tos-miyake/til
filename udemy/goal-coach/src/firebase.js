import * as firebase from 'firebase';

const config = {
  apiKey: "AIzaSyDBaJN6EIldsZnAZPvMjbXenyp9KIxL-r0",
  authDomain: "goal-coach-d9931.firebaseapp.com",
  databaseURL: "https://goal-coach-d9931.firebaseio.com",
  projectId: "goal-coach-d9931",
  storageBucket: "goal-coach-d9931.appspot.com",
  messagingSenderId: "611189079854"
};

export const firebaseApp = firebase.initializeApp(config);
