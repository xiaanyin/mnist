#ifndef __NEURON_UTILS_H__
#define __NEURON_UTILS_H__

#include <stdlib.h>
#include <math.h>
#include <iostream>
#include <iomanip>

using namespace std;

#define PROGRESS_DISP_CHAR '*'
/************************************************************************
* some random number functions.
************************************************************************/

//returns a random integer between x and y
inline int RandInt(int x, int y)
{ 
	return rand() % (y - x + 1) + x; 
}

//returns a random float between zero and 1
inline double RandFloat()
{ 
	return (rand()) / (RAND_MAX + 1.0); 
}

//returns a random bool
inline bool RandBool()
{
	return RandInt(0, 1) ? true : false;
}

//returns a random float in the range -1 < n < 1
inline double RandomClamped()
{ 
	return rand() % 1000 * 0.001 - 0.5;
	//return RandFloat() - RandFloat(); 
}


//Gaussian distribution
inline double RandGauss()
{
	static int	  iset = 0;
	static double gset = 0;
	double fac = 0, rsq = 0, v1 = 0, v2 = 0;

	if (iset == 0)
	{
		do
		{
			v1 = 2.0*RandFloat() - 1.0;
			v2 = 2.0*RandFloat() - 1.0;
			rsq = v1*v1 + v2*v2;
		} while (rsq >= 1.0 || rsq == 0.0);

		fac = sqrt(-2.0*log(rsq) / rsq);
		gset = v1*fac;
		iset = 1;
		return v2*fac;
	}
	else
	{
		iset = 0;
		return gset;
	}
}

inline void showSeparatorLine(char seprator, int lineSize)
{
	cout << setfill(seprator) << setw(lineSize) << seprator<<endl;
}


uint64_t timeNowMs();


class progressDisplay
{
public:
	progressDisplay(int maxValue);
	~progressDisplay();

	progressDisplay& operator++();
	progressDisplay& operator++(int);

	void updateProgress(int progressValue);

private:
	int mMaxValue;
	int mProgValue;
	int mMaxStarCnt;
	int mCurStarCnt;
	int mNumStarPer10Percent;
};

#endif // !__NEURON_UTILS_H__

