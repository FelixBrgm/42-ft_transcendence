// axios.js (create a separate file)
import axios from 'axios';

const instance = axios.create({
  withCredentials: true, // Send cookies with requests
  baseURL: 'http:127.0.0.1:8080', // Set your API base URL
});

export default instance;
