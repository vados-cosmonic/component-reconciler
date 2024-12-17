// Code generated by wit-bindgen-go. DO NOT EDIT.

package reconciler

import (
	"go.bytecodealliance.org/cm"
)

// This file contains wasmimport and wasmexport declarations for "example:reconciler@0.1.0".

//go:wasmexport reconcile
//export reconcile
func wasmexport_Reconcile(object0 *uint8, object1 uint32) (result *cm.Result[ReconcileResultShape, ReconcileResult, ReconcileError]) {
	object := cm.LiftString[string]((*uint8)(object0), (uint32)(object1))
	result_ := Exports.Reconcile(object)
	result = &result_
	return
}
