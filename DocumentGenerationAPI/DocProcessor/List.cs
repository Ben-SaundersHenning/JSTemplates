using DocumentFormat.OpenXml.Wordprocessing;

namespace DocProcessor;

public class List
{

  public Numbering test()
  {
    
      Numbering element = 
        new Numbering(
          new AbstractNum(
            new Level(
              new NumberingFormat() { Val = NumberFormatValues.Decimal },
              new LevelText() { Val = "%1."}
            ) { LevelIndex = 0 }
          ) { AbstractNumberId = 1 },
          new NumberingInstance(
            new AbstractNumId() { Val = 1 }
          ) { NumberID = 1 });

    return element;

  }

}