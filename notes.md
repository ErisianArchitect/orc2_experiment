### LLVMOrcAbsoluteSymbols
### LLVMOrcCreateCustomCAPIDefinitionGenerator
### LLVMOrcCreateCustomMaterializationUnit
### LLVMOrcCreateDumpObjects
### LLVMOrcCreateDynamicLibrarySearchGeneratorForPath
### LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess
### LLVMOrcCreateLocalIndirectStubsManager
### LLVMOrcCreateLocalLazyCallThroughManager
### LLVMOrcCreateNewThreadSafeContext
### LLVMOrcCreateNewThreadSafeModule
### LLVMOrcCreateStaticLibrarySearchGeneratorForPath
### LLVMOrcDisposeCSymbolFlagsMap
### LLVMOrcDisposeDefinitionGenerator
### LLVMOrcDisposeDumpObjects
### LLVMOrcDisposeIndirectStubsManager
### LLVMOrcDisposeJITTargetMachineBuilder
### LLVMOrcDisposeLazyCallThroughManager
### LLVMOrcDisposeMaterializationResponsibility
### LLVMOrcDisposeMaterializationUnit
### LLVMOrcDisposeObjectLayer
### LLVMOrcDisposeSymbols
### LLVMOrcDisposeThreadSafeContext
### LLVMOrcDisposeThreadSafeModule
### LLVMOrcDumpObjects_CallOperator
### LLVMOrcExecutionSessionCreateBareJITDylib
### LLVMOrcExecutionSessionCreateJITDylib
### LLVMOrcExecutionSessionGetJITDylibByName
### LLVMOrcExecutionSessionGetSymbolStringPool
### LLVMOrcExecutionSessionIntern
### LLVMOrcExecutionSessionLookup
### LLVMOrcExecutionSessionSetErrorReporter
### LLVMOrcIRTransformLayerEmit
### LLVMOrcIRTransformLayerSetTransform
### LLVMOrcJITDylibAddGenerator
### LLVMOrcJITDylibClear
### LLVMOrcJITDylibCreateResourceTracker
### LLVMOrcJITDylibDefine
### LLVMOrcJITDylibGetDefaultResourceTracker
### LLVMOrcJITTargetMachineBuilderCreateFromTargetMachine
### LLVMOrcJITTargetMachineBuilderDetectHost
### LLVMOrcJITTargetMachineBuilderGetTargetTriple
### LLVMOrcJITTargetMachineBuilderSetTargetTriple
### LLVMOrcLazyReexports
### LLVMOrcLookupStateContinueLookup
### LLVMOrcMaterializationResponsibilityAddDependencies
### LLVMOrcMaterializationResponsibilityAddDependenciesForAll
### LLVMOrcMaterializationResponsibilityDefineMaterializing
### LLVMOrcMaterializationResponsibilityDelegate
### LLVMOrcMaterializationResponsibilityFailMaterialization
### LLVMOrcMaterializationResponsibilityGetExecutionSession
### LLVMOrcMaterializationResponsibilityGetInitializerSymbol
### LLVMOrcMaterializationResponsibilityGetRequestedSymbols
### LLVMOrcMaterializationResponsibilityGetSymbols
### LLVMOrcMaterializationResponsibilityGetTargetDylib
### LLVMOrcMaterializationResponsibilityNotifyEmitted
### LLVMOrcMaterializationResponsibilityNotifyResolved
### LLVMOrcMaterializationResponsibilityReplace
### LLVMOrcObjectLayerAddObjectFile
### LLVMOrcObjectLayerAddObjectFileWithRT
### LLVMOrcObjectLayerEmit
### LLVMOrcObjectTransformLayerSetTransform
### LLVMOrcReleaseResourceTracker
### LLVMOrcReleaseSymbolStringPoolEntry
### LLVMOrcResourceTrackerRemove
### LLVMOrcResourceTrackerTransferTo
### LLVMOrcRetainSymbolStringPoolEntry
### LLVMOrcSymbolStringPoolClearDeadEntries
### LLVMOrcSymbolStringPoolEntryStr
### LLVMOrcThreadSafeContextGetContext
### LLVMOrcThreadSafeModuleWithModuleDo

# LLJIT
### LLVMOrcCreateLLJIT
### LLVMOrcCreateLLJITBuilder
### LLVMOrcDisposeLLJIT
### LLVMOrcDisposeLLJITBuilder
### LLVMOrcLLJITAddLLVMIRModule
### LLVMOrcLLJITAddLLVMIRModuleWithRT
### LLVMOrcLLJITAddObjectFile
### LLVMOrcLLJITAddObjectFileWithRT
### LLVMOrcLLJITBuilderSetJITTargetMachineBuilder
### LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator
### LLVMOrcLLJITEnableDebugSupport
    Install the plugin that submits debug objects to the executor.
### LLVMOrcLLJITGetDataLayoutStr
### LLVMOrcLLJITGetExecutionSession
### LLVMOrcLLJITGetGlobalPrefix
### LLVMOrcLLJITGetIRTransformLayer
### LLVMOrcLLJITGetMainJITDylib
### LLVMOrcLLJITGetObjLinkingLayer
### LLVMOrcLLJITGetObjTransformLayer
### LLVMOrcLLJITGetTripleString
### LLVMOrcLLJITLookup
### LLVMOrcLLJITMangleAndIntern

```
llvm-sys-80 = { package = "llvm-sys", version = "80.3", optional = true }
llvm-sys-90 = { package = "llvm-sys", version = "90.2.2", optional = true }
llvm-sys-100 = { package = "llvm-sys", version = "100.2.4", optional = true }
llvm-sys-110 = { package = "llvm-sys", version = "110.0.4", optional = true }
llvm-sys-120 = { package = "llvm-sys", version = "120.3.2", optional = true }
llvm-sys-130 = { package = "llvm-sys", version = "130.1.2", optional = true }
llvm-sys-140 = { package = "llvm-sys", version = "140.1.3", optional = true }
llvm-sys-150 = { package = "llvm-sys", version = "150.2.1", optional = true }
llvm-sys-160 = { package = "llvm-sys", version = "160.2.1", optional = true }
llvm-sys-170 = { package = "llvm-sys", version = "170.2.0", optional = true }
llvm-sys-181 = { package = "llvm-sys", version = "181.2.0", optional = true }
llvm-sys-191 = { package = "llvm-sys", version = "191.0.0", optional = true }
llvm-sys-201 = { package = "llvm-sys", version = "201.0.0", optional = true }
```

# orc versions:
80..=110 (api is exactly the same)
    llvm_sys::orc
120
    llvm_sys::orc2
130
    llvm_sys::orc2
140
    llvm_sys::orc2
150..=160
    llvm_sys::orc2