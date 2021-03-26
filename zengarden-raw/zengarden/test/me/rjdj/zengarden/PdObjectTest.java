/*
 *  Copyright 2010,2011,2012 Reality Jockey, Ltd.
 *                 info@rjdj.me
 *                 http://rjdj.me/
 * 
 *  This file is part of ZenGarden.
 *
 *  ZenGarden is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Lesser General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  ZenGarden is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser General Public License for more details.
 *  
 *  You should have received a copy of the GNU Lesser General Public License
 *  along with ZenGarden.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

package me.rjdj.zengarden;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.fail;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;

/**
 * This class is a test suite for all message objects.
 * 
 * @author Martin Roth (mhroth@rjdj.me)
 */
public class PdObjectTest implements ZenGardenListener {
  
  private static final int BLOCK_SIZE = 64;
  private static final int NUM_INPUT_CHANNELS = 2;
  private static final int NUM_OUTPUT_CHANNELS = 2;
  private static final float SAMPLE_RATE = 44100.0f;
  private static final short[] INPUT_BUFFER = new short[BLOCK_SIZE * NUM_INPUT_CHANNELS];
  private static final short[] OUTPUT_BUFFER = new short[BLOCK_SIZE * NUM_OUTPUT_CHANNELS];
  private static final String TEST_PATHNAME = "./test";
  
  private StringBuilder printBuffer;

  @Before
  public void setUp() throws Exception {
    printBuffer = new StringBuilder();
  }

  @After
  public void tearDown() throws Exception {
    // nothing to do
  }

  @Test
  public void testDspPrint() {
    genericMessageTest("DspPrint.pd");
  }

  @Test
  public void testMessageAdd() {
    genericMessageTest("MessageAdd.pd");
  }

  @Test
  public void testMessageAbsoluteValue() {
    genericMessageTest("MessageAbsoluteValue.pd");
  }

  @Test
  public void testMessageArcTangent() {
    genericMessageTest("MessageArcTangent.pd");
  }

  @Test
  public void testMessageArcTangent2() {
    genericMessageTest("MessageArcTangent2.pd");
  }

  @Test
  public void testMessageBang() {
    genericMessageTest("MessageBang.pd");
  }

  @Test
  public void testMessageChange() {
    genericMessageTest("MessageChange.pd");
  }

  @Test
  public void testMessageClip() {
    genericMessageTest("MessageClip.pd");
  }

  @Test
  public void testMessageCosine() {
    genericMessageTest("MessageCosine.pd");
  }
  
  @Test
  public void testMessageDiv() {
    genericMessageTest("MessageDiv.pd");
  }

  @Test
  public void testMessageDivide() {
    genericMessageTest("MessageDivide.pd");
  }

  @Test
  public void testMessageDbToPow() {
    genericMessageTest("MessageDbToPow.pd");
  }

  @Test
  public void testMessageDbToRms() {
    genericMessageTest("MessageDbToRms.pd");
  }

  @Test
  public void testMessageDelay() {
    genericMessageTest("MessageDelay.pd", 2000.0f);
  }

  @Test
  public void testMessageEqualsEquals() {
    genericMessageTest("MessageEqualsEquals.pd");
  }
  
  @Test
  public void testMessageExp() {
    genericMessageTest("MessageExp.pd");
  }

  @Test
  public void testMessageFloat() {
    genericMessageTest("MessageFloat.pd");
  }

  @Test
  public void testMessageFrequencyToMidi() {
    genericMessageTest("MessageFrequencyToMidi.pd");
  }

  @Test
  public void testMessageGreaterThan() {
    genericMessageTest("MessageGreaterThan.pd");
  }

  @Test
  public void testMessageGreaterThanOrEqualTo() {
    genericMessageTest("MessageGreaterThanOrEqualTo.pd");
  }
  
  @Test
  public void testMessageInletOutlet() {
    genericMessageTest("MessageInletOutlet.pd");
  }
  
  @Test
  public void testMessageInteger() {
    genericMessageTest("MessageInteger.pd");
  }

  @Test
  public void testMessageMessageBox() {
    genericMessageTest("MessageMessageBox.pd");
  }

  @Test
  public void testMessageLessThan() {
    genericMessageTest("MessageLessThan.pd");
  }
  
  @Test
  public void testMessageLessThanOrEqualTo() {
    genericMessageTest("MessageLessThanOrEqualTo.pd");
  }
  
  @Test
  public void testMessageListAppend() {
    genericMessageTest("MessageListAppend.pd");
  }
  
  @Test
  public void testMessageListLength() {
    genericMessageTest("MessageListLength.pd");
  }

  @Test
  public void testMessageLoadbang() {
    genericMessageTest("MessageLoadbang.pd");
  }

  @Test
  public void testMessageLog() {
    genericMessageTest("MessageLog.pd");
  }
  
  @Test
  public void testMessageLogicalAnd() {
    genericMessageTest("MessageLogicalAnd.pd");
  }
  
  @Test
  public void testMessageLogicalOr() {
    genericMessageTest("MessageLogicalOr.pd");
  }
  
  @Test
  public void testMessageMakefilename() {
    genericMessageTest("MessageMakefilename.pd");
  }  

  @Test
  public void testMessageMaximum() {
    genericMessageTest("MessageMaximum.pd");
  }  

  @Test
  public void testMessageMetro() {
    genericMessageTest("MessageMetro.pd", 11000.0f);
  } 

  @Test
  public void testMessageMinimum() {
    genericMessageTest("MessageMinimum.pd");
  }  

  @Test
  public void testMessageModulus() {
    genericMessageTest("MessageModulus.pd");
  }

  @Test
  public void testMessageMoses() {
    genericMessageTest("MessageMoses.pd");
  }

  @Test
  public void testMessageMultiply() { 
    genericMessageTest("MessageMultiply.pd");
  }

  @Test
  public void testMessageLine() { 
    genericMessageTest("MessageLine.pd", 3000.0f);
  }

  @Test
  public void testMessageNotEquals() {
    genericMessageTest("MessageNotEquals.pd");
  }
  
  @Test
  public void testMessagePack() {
    genericMessageTest("MessagePack.pd");
  }

  @Test
  public void testMessagePipe() {
    genericMessageTest("MessagePipe.pd", 2000.0f);
  }

  @Test
  public void testMessagePoly() {
    genericMessageTest("MessagePoly.pd");
  }
	
  @Test
  public void testMessagePow() {
    genericMessageTest("MessagePow.pd");
  }
  
  @Test
  public void testMessagePrint() {
    genericMessageTest("MessagePrint.pd");
  }

  @Test
  public void testMessageRandom() {
    genericMessageTest("MessageRandom.pd");
  }
  
  @Test
  public void testMessageReceive() {
    genericMessageTest("MessageReceive.pd");
  }
	
  @Test
  public void testMessageReminder() {
    genericMessageTest("MessageRemainder.pd");
  }

  @Test
  public void testMessageRmsToDb() {
    genericMessageTest("MessageRmsToDb.pd");
  }
  
  @Test
  public void testMessageRoute() {
    genericMessageTest("MessageRoute.pd");
  }

  @Test
  public void testMessageSelect() {
    genericMessageTest("MessageSelect.pd");
  }	
	
  @Test
  public void testMessageSend() {
    genericMessageTest("MessageSend.pd");
  }
  
  @Test
  public void testMessageSend_variable() {
    genericMessageTest("MessageSend_variable.pd");
  }

  @Test
  public void testMessageSine() {
    genericMessageTest("MessageSine.pd");
  }

  @Test
  public void testMessageSpigot() {
    genericMessageTest("MessageSpigot.pd");
  }
	
  @Test
  public void testMessageSqrt() {
    genericMessageTest("MessageSqrt.pd");
  }
	
  @Test
  public void testMessageSubtract() {
    genericMessageTest("MessageSubtract.pd");
  }

  @Test
  public void testMessageSwap() {
	genericMessageTest("MessageSwap.pd");
  }
	
  @Test
  public void testMessageSymbol() {
    genericMessageTest("MessageSymbol.pd");
  }
	
  @Test
  public void testMessageTable() {
    genericMessageTest("MessageTable.pd");
  }

  @Test
  public void testMessageTangent() {
    genericMessageTest("MessageTangent.pd");
  }
	
  @Test
  public void testMessageTimer() {
    genericMessageTest("MessageTimer.pd", 1247.0f);
  }

  @Test
  public void testMessageToggle() {
    genericMessageTest("MessageToggle.pd");
  }
  
  @Test
  public void testMessageTrigger() {
    genericMessageTest("MessageTrigger.pd");
  }
  
  @Test
  public void testMessageUnpack() {
    genericMessageTest("MessageUnpack.pd");
  }

  @Test
  public void testMessageUntil() {
    genericMessageTest("MessageUntil.pd");
  }
  
  @Test
  public void testMessageValue() {
    genericMessageTest("MessageValue.pd");
  }

  @Test
  public void testMessageWrap() {
    genericMessageTest("MessageWrap.pd");
  }
  
  @Test
  public void testMultipleReceiversWithSameNameAllReceiveMessage() {
    genericMessageTest("multiple-receiver-test.pd");
  }
  
  /**
   * Executes the generic message test for at least the given minimum runtime (in milliseconds).
   */
  private void genericMessageTest(String testFilename, float minmumRuntimeMs) {
    ZGContext context = new ZGContext(NUM_INPUT_CHANNELS, NUM_OUTPUT_CHANNELS, BLOCK_SIZE, SAMPLE_RATE);
    context.addListener(this);
    ZGGraph graph = context.newGraph(new File(TEST_PATHNAME, testFilename));
    graph.attach();
    
    // process at least as many blocks as necessary to cover the givenruntime
    int numBlocksToProcess = (int) (Math.floor(((minmumRuntimeMs/1000.0f)*SAMPLE_RATE)/BLOCK_SIZE)+1);
    for (int i = 0; i < numBlocksToProcess; i++) {
      context.process(INPUT_BUFFER, OUTPUT_BUFFER);
    }
    
    String goldenOutput = readTextFile(new File(TEST_PATHNAME,
        testFilename.split("\\.")[0] + ".golden.txt"));
    
    // ensure that message standard output is same as golden file
    assertEquals(goldenOutput, printBuffer.toString());
  }
  
  /**
   * Encompasses a generic test for message objects. It processes the graph once and compares the
   * standard output to the golden file, and ensures that the error output is empty.
   * @param testFilename
   */
  private void genericMessageTest(String testFilename) {
    genericMessageTest(testFilename, 0.0f);
  }
  
  private String readTextFile(File file) {
    StringBuilder contents = new StringBuilder();
    try {
      BufferedReader input = new BufferedReader(new FileReader(file));
      try {
        String line = null;
        while (( line = input.readLine()) != null){
          contents.append(line);
          contents.append(System.getProperty("line.separator"));
        }
      }
      finally {
        input.close();
      }
    }
    catch (IOException ioe){
      fail(ioe.toString());
    }
    return contents.toString();
  }

  public void onPrintErr(String message) {
    // must append new line because message does not have it by default
    //printBuffer.append(message);
    //printBuffer.append(System.getProperty("line.separator"));
  }

  public void onPrintStd(String message) {
    printBuffer.append(message);
    printBuffer.append(System.getProperty("line.separator"));
  }

  public void onMessage(String receiverName, Message message) {
    // nothing to do 
  }

}
