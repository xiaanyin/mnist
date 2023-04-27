#include <chrono>
#include "neuron_utils.h"



progressDisplay::progressDisplay(int maxValue)
    :mMaxValue(maxValue),
    mNumStarPer10Percent(5),
    mProgValue(0),
    mMaxStarCnt(50),
    mCurStarCnt(0)
{
    cout << "0%";
    for (size_t i = 1; i < 10; i++)
    {
        size_t progValue = i * 10;
        cout << setfill(' ') << setw(mNumStarPer10Percent) << progValue;

        if (i == 9)
        {
            /** 100%  and 50 mush than 2 char, so we need + 2 */
            cout << setfill(' ') << setw(mNumStarPer10Percent + 2) << "100%" << endl;
        }
    }

    for (size_t i = 0; i < 10; i++)
    {
        cout << '|' << setfill('-') << setw(mNumStarPer10Percent);
    }
    cout << '|' << endl;
}

progressDisplay::~progressDisplay()
{
    cout << endl;
}

progressDisplay& progressDisplay::operator++()
{
    updateProgress(mProgValue + 1);

    return *this;
}

progressDisplay& progressDisplay::operator++(int)
{
    updateProgress(mProgValue + 1);

    return *this;
}

void progressDisplay::updateProgress(int progressValue)
{
    mProgValue = progressValue;
    int progValue = mProgValue * mMaxStarCnt / mMaxValue;

    if (progValue > mCurStarCnt)
    {
        cout <<'\r' << setfill(PROGRESS_DISP_CHAR) << setw(progValue) << PROGRESS_DISP_CHAR;
        mCurStarCnt = progValue;

        if (mCurStarCnt >= mMaxStarCnt)
        {
            cout << PROGRESS_DISP_CHAR << endl;
        }
    }
}

uint64_t timeNowMs()
{
    std::chrono::time_point<std::chrono::system_clock> p2 =
        std::chrono::system_clock::now();
    return std::chrono::duration_cast<std::chrono::milliseconds>
        (p2.time_since_epoch()).count();
}