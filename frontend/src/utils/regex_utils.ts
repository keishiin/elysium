const emailRegex = /^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/;
const passwordRegex =
  /^(?=.*[A-Z])(?=.*[a-z])(?=.*\d)(?=.*[@#$%^&+=!])[A-Za-z\d@#$%^&+=!]{8,}$/;

export function isValidEmail(email: string) {
  return emailRegex.test(email);
}

export function isValidPassword(password: string) {
  return passwordRegex.test(password);
}

export default {
  isValidEmail,
  isValidPassword,
};
