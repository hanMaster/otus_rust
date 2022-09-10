#ifndef _SOCK_CLIENT_H
#define _SOCK_CLIENT_H

#ifdef __cplusplus
extern "C"{
#endif

    struct SocketState {
        int state;
        double power;
        int error;
    };

    SocketState sync();
    int switch_on();
    int switch_off();

#ifdef __cplusplus
}
#endif
#endif