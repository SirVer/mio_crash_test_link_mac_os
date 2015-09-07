#!/usr/bin/env python
# -*- coding: utf-8 -*-

import ctypes

crash_test = ctypes.cdll.LoadLibrary("target/debug/libcrash_test.dylib")
crash_test.connect()
