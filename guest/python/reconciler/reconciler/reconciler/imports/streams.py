from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from types import TracebackType
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some
from ..imports import poll
from ..imports import error


@dataclass
class StreamError_LastOperationFailed:
    value: error.Error


@dataclass
class StreamError_Closed:
    pass


StreamError = Union[StreamError_LastOperationFailed, StreamError_Closed]


class InputStream:
    
    def read(self, len: int) -> bytes:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def blocking_read(self, len: int) -> bytes:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def skip(self, len: int) -> int:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def blocking_skip(self, len: int) -> int:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def subscribe(self) -> poll.Pollable:
        raise NotImplementedError
    def __enter__(self) -> Self:
        """Returns self"""
        return self
                                
    def __exit__(self, exc_type: type[BaseException] | None, exc_value: BaseException | None, traceback: TracebackType | None) -> bool | None:
        """
        Release this resource.
        """
        raise NotImplementedError


class OutputStream:
    
    def check_write(self) -> int:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def write(self, contents: bytes) -> None:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def blocking_write_and_flush(self, contents: bytes) -> None:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def flush(self) -> None:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def blocking_flush(self) -> None:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def subscribe(self) -> poll.Pollable:
        raise NotImplementedError
    def write_zeroes(self, len: int) -> None:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def blocking_write_zeroes_and_flush(self, len: int) -> None:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def splice(self, src: InputStream, len: int) -> int:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def blocking_splice(self, src: InputStream, len: int) -> int:
        """
        Raises: `reconciler.types.Err(reconciler.imports.streams.StreamError)`
        """
        raise NotImplementedError
    def __enter__(self) -> Self:
        """Returns self"""
        return self
                                
    def __exit__(self, exc_type: type[BaseException] | None, exc_value: BaseException | None, traceback: TracebackType | None) -> bool | None:
        """
        Release this resource.
        """
        raise NotImplementedError



