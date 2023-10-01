runG:
	dotnet run --project ./DocumentGenerationAPI/DocumentGenerationAPI.csproj & cargo tauri dev

buildG:
	dotnet build ./GenerationAPI/DocGen/DocGen.csproj

cleanG:
	dotnet clean ./GenerationAPI/DocGen/DocGen.csproj

