#include "neuron_utils.h"
#include "neuron.h"

using namespace std;

/************************************************************************
 * neuron                                                                      
 ************************************************************************/


/************************************************************************
 * neuron layer                                                                      
 ************************************************************************/

neuronLayer::neuronLayer(int numNeurons, int numInputsPerNeuron)
	:mNumNeurons(numNeurons),
	mNumInputsPerNeuron(numInputsPerNeuron)
{
	mWeights = new double*[mNumNeurons];
	mOutActivations = new double[mNumNeurons];
	mOutErrors = new double[mNumNeurons];

	reset();
}

neuronLayer::neuronLayer(neuronLayer& nl)
	:neuronLayer(nl.mNumNeurons, nl.mNumInputsPerNeuron)
{
	int copySize = mNumNeurons * sizeof(double);

	memcpy(mOutActivations, nl.mOutActivations, copySize);
	memcpy(mOutErrors, nl.mOutErrors, copySize);

	for (int i = 0; i < mNumNeurons; i++)
	{
		memcpy(mWeights[i], nl.mWeights[i], copySize);
	}
}

neuronLayer::~neuronLayer()
{
	/** release weights */
	for (int i = 0; i < mNumNeurons; i++)
	{
		delete []mWeights[i];
	}

	delete []mWeights;

	/** release activations */
	delete []mOutActivations;

	/** release errors */
	delete []mOutErrors;
}

void neuronLayer::reset()
{
	memset(mOutActivations, 0, mNumNeurons * sizeof(double));
	memset(mOutErrors, 0, mNumNeurons * sizeof(double));

	for (int i = 0; i < mNumNeurons; ++i)
	{
		//we need an additional weight for the bias hence the +1
		int numWeights = mNumInputsPerNeuron + 1;
		double* curWeights = new double[numWeights];
		mWeights[i] = curWeights;

		for (int w = 0; w < numWeights; w++)
		{
			//set up the weights with an initial random value
			double temp = RandomClamped();
			curWeights[w] = temp;
		}
	}
}

