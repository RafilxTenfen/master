#include "Stack_Public.h"
#include "Stack_Private.h"

void push (int x)
{
    if (s.top == (MAXSIZE - 1))
    {
        printf ("\nA pilha está cheia");
        return;
    }
    else
    {
        s.top = s.top + 1;
        s.stk[s.top] = x;
    }
    return;
}

int pop ()
{
    int x;
    if (s.top == - 1)
    {
        printf ("\A pilha está vazia");
        return (s.top);
    }
    else
    {
        x = s.stk[s.top];
        printf ("\nElemento retirado = %d", s.stk[s.top]);
        printf ("\n");
        s.top = s.top - 1;
    }
    return(x);
}

void display ()
{
    int i;
    if (s.top == -1)
    {
        printf ("\nA pilha está vazia");
        return;
    }
    else
    {
        printf ("\nElementos na pilha:");
        for (i = s.top; i > 0; i--)
        {
            printf ("\n%d", s.stk[i]);
        }
    }
    printf ("\n");
}
int top ()
{
    int x;
    if (s.top == - 1)
    {
        printf ("\A pilha está vazia");
        return (s.top);
    }
    else
    {
        x = s.stk[s.top];
        printf ("\nElemento TOP = %d", s.stk[s.top]);
        printf ("\n");
        s.top = s.top;
    }
    return(x);
}
