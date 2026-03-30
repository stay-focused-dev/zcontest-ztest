#include <stdio.h>
#include <memory.h>
#define maxn 2147483647
#define maxr 33333333
#define maxA (maxn/30+2)
#define maxk (100000)
#define maxc (maxn/(30*maxk)+1)
#define maxl (maxr/2/maxc+100)
#define uchar unsigned char
int maxa=maxk;

uchar a[maxk+2];
uchar aaa[maxc][maxk+2];
uchar d[30]={0,1,0,0,0,0,0,2,0,0,0,4,0,8,0,0,0,16,0,32,0,0,0,64,0,0,0,0,0,128};

int j, i, l=0, _j1, I21, I41, I61, i2, n, s, J, I22, I42, I62;
int pr[5000], S[5000], J1[5000], k, A[50000], _a;

#define o(y,t,z) if (s<=z) { s=0, _j1+=y; if (_j1<maxk) a[_j1]|=(uchar)t; else {_j1-=y, s=z; break;} }

void check(int I)
{
 int i=pr[I];
 s=S[I];
 _j1=J1[I];
 I21=i/15;
 I41=(i+i)/15;
 I61=i/5;
 i2=i%30;
 if (i2==1)
 {
  I22=I21+1;
  while (_j1<maxk)
  {
   o(I22,1,0)
   o(I61,2,1)
   o(I41,4,2)
   o(I21,8,3)
   o(I41,16,4)
   o(I21,32,5)
   o(I41,64,6)
   o(I61,128,7)
  }
 }
 else if (i2==7)
 {
  I21++;
  I42=I41+1;
  while (_j1<maxk)
  {
   o(I61,32,0)
   o(I42,16,1)
   o(I21,1,2)
   o(I41,128,3)
   o(I21,8,4)
   o(I42,4,5)
   o(I61,64,6)
   o(I21,2,7)
  }
 }
 else if (i2==11)
 {
  I41++;
  I22=I21+1;
  while (_j1<maxk)
  {
   o(I41,1,0)
   o(I21,64,1)
   o(I41,2,2)
   o(I21,128,3)
   o(I41,8,4)
   o(I61,32,5)
   o(I22,4,6)
   o(I61,16,7)
  }
 }
 else if (i2==13)
 {
  I21++;
  I42=I41+1;
  I61++;
  while (_j1<maxk)
  {
   o(I21,32,0)
   o(I42,4,1)
   o(I21,2,2)
   o(I41,128,3)
   o(I61,16,4)
   o(I21,8,5)
   o(I61,1,6)
   o(I41,64,7)
  }
 }
 else if (i2==17)
 {
  I42=I41+1;
  while (_j1<maxk)
  {
   o(I41,32,0)
   o(I21,64,1)
   o(I42,1,2)
   o(I61,8,3)
   o(I21,16,4)
   o(I61,128,5)
   o(I42,2,6)
   o(I21,4,7)
  }
 }
 else if (i2==19)
 {
  I22=I21+1;
  I61++;
  while (_j1<maxk)
  {
   o(I22,1,0)
   o(I41,16,1)
   o(I61,4,2)
   o(I21,32,3)
   o(I61,8,4)
   o(I41,128,5)
   o(I22,2,6)
   o(I41,64,7)
  }
 }
 else if (i2==23)
 {
  I42=I41+1;
  I61++;
  while (_j1<maxk)
  {
   o(I41,32,0)
   o(I61,2,1)
   o(I21,64,2)
   o(I61,4,3)
   o(I41,8,4)
   o(I21,128,5)
   o(I42,1,6)
   o(I21,16,7)
  }
 }
 else if (i2==29)
 {
  I22=I21+1;
  I41++;
  I61++;
  while (_j1<maxk)
  {
   o(I61,1,0)
   o(I21,128,1)
   o(I61,64,2)
   o(I41,32,3)
   o(I22,16,4)
   o(I41,8,5)
   o(I22,4,6)
   o(I41,2,7)
  }
 }
 J1[I]=_j1-maxk;
 S[I]=s;
}

int main(void)
{
 int i, j, t;
 for (i=3; i<=46340; i+=2)
  if (!A[i])
  {
   i2=i+i;
   for (j=i*i; j<=46340; j+=i2)
    A[j]=1;
   if (i<11) continue;
   i2=i%30;
   if (i2==1) J1[k]=(i*i-2*i)/30;
   if (i2==7) J1[k]=(i*i-6*i)/30;
   if (i2==11) J1[k]=(i*i-4*i)/30;
   if (i2==13) J1[k]=(i*i-2*i)/30;
   if (i2==17) J1[k]=(i*i-4*i)/30;
   if (i2==19) J1[k]=(i*i-2*i)/30;
   if (i2==23) J1[k]=(i*i-4*i)/30;
   if (i2==29) J1[k]=(i*i-6*i)/30;
   pr[k++]=i;
  }

 for (_a=0; (maxa-maxk)*30>=0; _a++, maxa+=maxk)
 {
  for (i=0; i<k; i++)
   check(i);
  memcpy(aaa[_a], a, sizeof(a));
  memset(a,0,sizeof(a));
 }

 putchar('0');
 n=1;
 _j1=0;
 for (i=1; i<maxr; i++)
 {
  n=(n+1234567890)&maxn;
  if (!d[(n%30)] || n%7==0) a[_j1]='0';
  else
  {
     _a=n/(30*maxk);
     t=n%(30*maxk);
     if (!(aaa[_a][t/30]&d[t%30])) a[_j1]='1';
     else a[_j1]='0';
  }
  if (_j1==maxk)
  {
     printf("%s", a);
     _j1=0;
  }
  else _j1++;
 }
 a[_j1]=0;
 printf("%s", a);
 return 0;
}