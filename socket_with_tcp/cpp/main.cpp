#include <iostream>
#include "sock_client.h"

using namespace std;

int main()
{
    int result_on = switch_on();
    cout << "Switch on result: " << result_on << endl;

    SocketState s = sync();

    if (s.error == 0) {
        cout << "Switch state: " << s.state << endl;
        cout << "Switch power: " << s.power << endl;
    } else {
        cout << "Failed to get state" << endl;
    }

    int result_off = switch_off();
    cout << "Switch off result: " << result_off << endl;

    return 0;
}
