import * as generated from './generated';
import { client } from './generated';

export * from './generated';

export const sdk = {
  client,
  auth: {
    register: generated.AuthService.postRegister,
    login: generated.AuthService.postLogin,
    me: generated.AuthService.getMe,
    logout: generated.AuthService.postLogout,
  },
  class: {
    list: generated.ClassService.getClasses,
    create: generated.ClassService.postClasses,
  },
  behavior: {
    list: generated.BehaviorService.getBehavior,
    submitAi: generated.BehaviorService.postAiBehavior,
  },
  notification: {
    list: generated.NotificationService.getNotifications,
    markRead: generated.NotificationService.postMarkRead,
  }
};
