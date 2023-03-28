#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNC` reader - Function"]
pub type FUNC_R = crate::FieldReader<u8, FUNC_A>;
#[doc = "Function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: Cosine function"]
    Cosine = 0,
    #[doc = "1: Sine function"]
    Sine = 1,
    #[doc = "2: Phase function"]
    Phase = 2,
    #[doc = "3: Modulus function"]
    Modulus = 3,
    #[doc = "4: Arctangent function"]
    Arctangent = 4,
    #[doc = "5: Hyperbolic Cosine function"]
    HyperbolicCosine = 5,
    #[doc = "6: Hyperbolic Sine function"]
    HyperbolicSine = 6,
    #[doc = "7: Arctanh function"]
    Arctanh = 7,
    #[doc = "8: Natural Logarithm function"]
    NaturalLogarithm = 8,
    #[doc = "9: Square Root function"]
    SquareRoot = 9,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
impl FUNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::Cosine),
            1 => Some(FUNC_A::Sine),
            2 => Some(FUNC_A::Phase),
            3 => Some(FUNC_A::Modulus),
            4 => Some(FUNC_A::Arctangent),
            5 => Some(FUNC_A::HyperbolicCosine),
            6 => Some(FUNC_A::HyperbolicSine),
            7 => Some(FUNC_A::Arctanh),
            8 => Some(FUNC_A::NaturalLogarithm),
            9 => Some(FUNC_A::SquareRoot),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Cosine`"]
    #[inline(always)]
    pub fn is_cosine(&self) -> bool {
        *self == FUNC_A::Cosine
    }
    #[doc = "Checks if the value of the field is `Sine`"]
    #[inline(always)]
    pub fn is_sine(&self) -> bool {
        *self == FUNC_A::Sine
    }
    #[doc = "Checks if the value of the field is `Phase`"]
    #[inline(always)]
    pub fn is_phase(&self) -> bool {
        *self == FUNC_A::Phase
    }
    #[doc = "Checks if the value of the field is `Modulus`"]
    #[inline(always)]
    pub fn is_modulus(&self) -> bool {
        *self == FUNC_A::Modulus
    }
    #[doc = "Checks if the value of the field is `Arctangent`"]
    #[inline(always)]
    pub fn is_arctangent(&self) -> bool {
        *self == FUNC_A::Arctangent
    }
    #[doc = "Checks if the value of the field is `HyperbolicCosine`"]
    #[inline(always)]
    pub fn is_hyperbolic_cosine(&self) -> bool {
        *self == FUNC_A::HyperbolicCosine
    }
    #[doc = "Checks if the value of the field is `HyperbolicSine`"]
    #[inline(always)]
    pub fn is_hyperbolic_sine(&self) -> bool {
        *self == FUNC_A::HyperbolicSine
    }
    #[doc = "Checks if the value of the field is `Arctanh`"]
    #[inline(always)]
    pub fn is_arctanh(&self) -> bool {
        *self == FUNC_A::Arctanh
    }
    #[doc = "Checks if the value of the field is `NaturalLogarithm`"]
    #[inline(always)]
    pub fn is_natural_logarithm(&self) -> bool {
        *self == FUNC_A::NaturalLogarithm
    }
    #[doc = "Checks if the value of the field is `SquareRoot`"]
    #[inline(always)]
    pub fn is_square_root(&self) -> bool {
        *self == FUNC_A::SquareRoot
    }
}
#[doc = "Field `FUNC` writer - Function"]
pub type FUNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, FUNC_A, 4, O>;
impl<'a, const O: u8> FUNC_W<'a, O> {
    #[doc = "Cosine function"]
    #[inline(always)]
    pub fn cosine(self) -> &'a mut W {
        self.variant(FUNC_A::Cosine)
    }
    #[doc = "Sine function"]
    #[inline(always)]
    pub fn sine(self) -> &'a mut W {
        self.variant(FUNC_A::Sine)
    }
    #[doc = "Phase function"]
    #[inline(always)]
    pub fn phase(self) -> &'a mut W {
        self.variant(FUNC_A::Phase)
    }
    #[doc = "Modulus function"]
    #[inline(always)]
    pub fn modulus(self) -> &'a mut W {
        self.variant(FUNC_A::Modulus)
    }
    #[doc = "Arctangent function"]
    #[inline(always)]
    pub fn arctangent(self) -> &'a mut W {
        self.variant(FUNC_A::Arctangent)
    }
    #[doc = "Hyperbolic Cosine function"]
    #[inline(always)]
    pub fn hyperbolic_cosine(self) -> &'a mut W {
        self.variant(FUNC_A::HyperbolicCosine)
    }
    #[doc = "Hyperbolic Sine function"]
    #[inline(always)]
    pub fn hyperbolic_sine(self) -> &'a mut W {
        self.variant(FUNC_A::HyperbolicSine)
    }
    #[doc = "Arctanh function"]
    #[inline(always)]
    pub fn arctanh(self) -> &'a mut W {
        self.variant(FUNC_A::Arctanh)
    }
    #[doc = "Natural Logarithm function"]
    #[inline(always)]
    pub fn natural_logarithm(self) -> &'a mut W {
        self.variant(FUNC_A::NaturalLogarithm)
    }
    #[doc = "Square Root function"]
    #[inline(always)]
    pub fn square_root(self) -> &'a mut W {
        self.variant(FUNC_A::SquareRoot)
    }
}
#[doc = "Field `PRECISION` reader - Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)"]
pub type PRECISION_R = crate::FieldReader<u8, PRECISION_A>;
#[doc = "Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRECISION_A {
    #[doc = "1: 4 iterations"]
    Iters4 = 1,
    #[doc = "2: 8 iterations"]
    Iters8 = 2,
    #[doc = "3: 12 iterations"]
    Iters12 = 3,
    #[doc = "4: 16 iterations"]
    Iters16 = 4,
    #[doc = "5: 20 iterations"]
    Iters20 = 5,
    #[doc = "6: 24 iterations"]
    Iters24 = 6,
    #[doc = "7: 28 iterations"]
    Iters28 = 7,
    #[doc = "8: 32 iterations"]
    Iters32 = 8,
    #[doc = "9: 36 iterations"]
    Iters36 = 9,
    #[doc = "10: 40 iterations"]
    Iters40 = 10,
    #[doc = "11: 44 iterations"]
    Iters44 = 11,
    #[doc = "12: 48 iterations"]
    Iters48 = 12,
    #[doc = "13: 52 iterations"]
    Iters52 = 13,
    #[doc = "14: 56 iterations"]
    Iters56 = 14,
    #[doc = "15: 60 iterations"]
    Iters60 = 15,
}
impl From<PRECISION_A> for u8 {
    #[inline(always)]
    fn from(variant: PRECISION_A) -> Self {
        variant as _
    }
}
impl PRECISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRECISION_A> {
        match self.bits {
            1 => Some(PRECISION_A::Iters4),
            2 => Some(PRECISION_A::Iters8),
            3 => Some(PRECISION_A::Iters12),
            4 => Some(PRECISION_A::Iters16),
            5 => Some(PRECISION_A::Iters20),
            6 => Some(PRECISION_A::Iters24),
            7 => Some(PRECISION_A::Iters28),
            8 => Some(PRECISION_A::Iters32),
            9 => Some(PRECISION_A::Iters36),
            10 => Some(PRECISION_A::Iters40),
            11 => Some(PRECISION_A::Iters44),
            12 => Some(PRECISION_A::Iters48),
            13 => Some(PRECISION_A::Iters52),
            14 => Some(PRECISION_A::Iters56),
            15 => Some(PRECISION_A::Iters60),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Iters4`"]
    #[inline(always)]
    pub fn is_iters4(&self) -> bool {
        *self == PRECISION_A::Iters4
    }
    #[doc = "Checks if the value of the field is `Iters8`"]
    #[inline(always)]
    pub fn is_iters8(&self) -> bool {
        *self == PRECISION_A::Iters8
    }
    #[doc = "Checks if the value of the field is `Iters12`"]
    #[inline(always)]
    pub fn is_iters12(&self) -> bool {
        *self == PRECISION_A::Iters12
    }
    #[doc = "Checks if the value of the field is `Iters16`"]
    #[inline(always)]
    pub fn is_iters16(&self) -> bool {
        *self == PRECISION_A::Iters16
    }
    #[doc = "Checks if the value of the field is `Iters20`"]
    #[inline(always)]
    pub fn is_iters20(&self) -> bool {
        *self == PRECISION_A::Iters20
    }
    #[doc = "Checks if the value of the field is `Iters24`"]
    #[inline(always)]
    pub fn is_iters24(&self) -> bool {
        *self == PRECISION_A::Iters24
    }
    #[doc = "Checks if the value of the field is `Iters28`"]
    #[inline(always)]
    pub fn is_iters28(&self) -> bool {
        *self == PRECISION_A::Iters28
    }
    #[doc = "Checks if the value of the field is `Iters32`"]
    #[inline(always)]
    pub fn is_iters32(&self) -> bool {
        *self == PRECISION_A::Iters32
    }
    #[doc = "Checks if the value of the field is `Iters36`"]
    #[inline(always)]
    pub fn is_iters36(&self) -> bool {
        *self == PRECISION_A::Iters36
    }
    #[doc = "Checks if the value of the field is `Iters40`"]
    #[inline(always)]
    pub fn is_iters40(&self) -> bool {
        *self == PRECISION_A::Iters40
    }
    #[doc = "Checks if the value of the field is `Iters44`"]
    #[inline(always)]
    pub fn is_iters44(&self) -> bool {
        *self == PRECISION_A::Iters44
    }
    #[doc = "Checks if the value of the field is `Iters48`"]
    #[inline(always)]
    pub fn is_iters48(&self) -> bool {
        *self == PRECISION_A::Iters48
    }
    #[doc = "Checks if the value of the field is `Iters52`"]
    #[inline(always)]
    pub fn is_iters52(&self) -> bool {
        *self == PRECISION_A::Iters52
    }
    #[doc = "Checks if the value of the field is `Iters56`"]
    #[inline(always)]
    pub fn is_iters56(&self) -> bool {
        *self == PRECISION_A::Iters56
    }
    #[doc = "Checks if the value of the field is `Iters60`"]
    #[inline(always)]
    pub fn is_iters60(&self) -> bool {
        *self == PRECISION_A::Iters60
    }
}
#[doc = "Field `PRECISION` writer - Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)"]
pub type PRECISION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSR_SPEC, u8, PRECISION_A, 4, O>;
impl<'a, const O: u8> PRECISION_W<'a, O> {
    #[doc = "4 iterations"]
    #[inline(always)]
    pub fn iters4(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters4)
    }
    #[doc = "8 iterations"]
    #[inline(always)]
    pub fn iters8(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters8)
    }
    #[doc = "12 iterations"]
    #[inline(always)]
    pub fn iters12(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters12)
    }
    #[doc = "16 iterations"]
    #[inline(always)]
    pub fn iters16(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters16)
    }
    #[doc = "20 iterations"]
    #[inline(always)]
    pub fn iters20(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters20)
    }
    #[doc = "24 iterations"]
    #[inline(always)]
    pub fn iters24(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters24)
    }
    #[doc = "28 iterations"]
    #[inline(always)]
    pub fn iters28(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters28)
    }
    #[doc = "32 iterations"]
    #[inline(always)]
    pub fn iters32(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters32)
    }
    #[doc = "36 iterations"]
    #[inline(always)]
    pub fn iters36(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters36)
    }
    #[doc = "40 iterations"]
    #[inline(always)]
    pub fn iters40(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters40)
    }
    #[doc = "44 iterations"]
    #[inline(always)]
    pub fn iters44(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters44)
    }
    #[doc = "48 iterations"]
    #[inline(always)]
    pub fn iters48(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters48)
    }
    #[doc = "52 iterations"]
    #[inline(always)]
    pub fn iters52(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters52)
    }
    #[doc = "56 iterations"]
    #[inline(always)]
    pub fn iters56(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters56)
    }
    #[doc = "60 iterations"]
    #[inline(always)]
    pub fn iters60(self) -> &'a mut W {
        self.variant(PRECISION_A::Iters60)
    }
}
#[doc = "Field `SCALE` reader - Scaling factor (2^-n for arguments, 2^n for results)"]
pub type SCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCALE` writer - Scaling factor (2^-n for arguments, 2^n for results)"]
pub type SCALE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `IEN` reader - Enable interrupt"]
pub type IEN_R = crate::BitReader<IEN_A>;
#[doc = "Enable interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN_A {
    #[doc = "0: Disable interrupt request generation"]
    Disabled = 0,
    #[doc = "1: Enable interrupt request generation"]
    Enabled = 1,
}
impl From<IEN_A> for bool {
    #[inline(always)]
    fn from(variant: IEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN_A {
        match self.bits {
            false => IEN_A::Disabled,
            true => IEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IEN_A::Enabled
    }
}
#[doc = "Field `IEN` writer - Enable interrupt"]
pub type IEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, IEN_A, O>;
impl<'a, const O: u8> IEN_W<'a, O> {
    #[doc = "Disable interrupt request generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IEN_A::Disabled)
    }
    #[doc = "Enable interrupt request generation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IEN_A::Enabled)
    }
}
#[doc = "Field `DMAREN` reader - Enable DMA wread channel"]
pub type DMAREN_R = crate::BitReader<DMAREN_A>;
#[doc = "Enable DMA wread channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAREN_A {
    #[doc = "0: No DMA channel reads are generated"]
    Disabled = 0,
    #[doc = "1: Read requests are generated on the DMA channel when RRDY flag is set"]
    Enabled = 1,
}
impl From<DMAREN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAREN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAREN_A {
        match self.bits {
            false => DMAREN_A::Disabled,
            true => DMAREN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAREN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAREN_A::Enabled
    }
}
#[doc = "Field `DMAREN` writer - Enable DMA wread channel"]
pub type DMAREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, DMAREN_A, O>;
impl<'a, const O: u8> DMAREN_W<'a, O> {
    #[doc = "No DMA channel reads are generated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAREN_A::Disabled)
    }
    #[doc = "Read requests are generated on the DMA channel when RRDY flag is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAREN_A::Enabled)
    }
}
#[doc = "Field `DMAWEN` reader - Enable DMA write channel"]
pub type DMAWEN_R = crate::BitReader<DMAWEN_A>;
#[doc = "Enable DMA write channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAWEN_A {
    #[doc = "0: No DMA channel writes are generated"]
    Disabled = 0,
    #[doc = "1: Write requests are generated on the DMA channel when no operation is pending"]
    Enabled = 1,
}
impl From<DMAWEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAWEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAWEN_A {
        match self.bits {
            false => DMAWEN_A::Disabled,
            true => DMAWEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAWEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAWEN_A::Enabled
    }
}
#[doc = "Field `DMAWEN` writer - Enable DMA write channel"]
pub type DMAWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, DMAWEN_A, O>;
impl<'a, const O: u8> DMAWEN_W<'a, O> {
    #[doc = "No DMA channel writes are generated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAWEN_A::Disabled)
    }
    #[doc = "Write requests are generated on the DMA channel when no operation is pending"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAWEN_A::Enabled)
    }
}
#[doc = "Field `NRES` reader - Number of results in the RDATA register"]
pub type NRES_R = crate::BitReader<NRES_A>;
#[doc = "Number of results in the RDATA register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NRES_A {
    #[doc = "0: Only single result value will be returned. After a single read RRDY will be automatically cleared"]
    Num1 = 0,
    #[doc = "1: Two return reads need to be performed. After two reads RRDY will be automatically cleared"]
    Num2 = 1,
}
impl From<NRES_A> for bool {
    #[inline(always)]
    fn from(variant: NRES_A) -> Self {
        variant as u8 != 0
    }
}
impl NRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRES_A {
        match self.bits {
            false => NRES_A::Num1,
            true => NRES_A::Num2,
        }
    }
    #[doc = "Checks if the value of the field is `Num1`"]
    #[inline(always)]
    pub fn is_num1(&self) -> bool {
        *self == NRES_A::Num1
    }
    #[doc = "Checks if the value of the field is `Num2`"]
    #[inline(always)]
    pub fn is_num2(&self) -> bool {
        *self == NRES_A::Num2
    }
}
#[doc = "Field `NRES` writer - Number of results in the RDATA register"]
pub type NRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, NRES_A, O>;
impl<'a, const O: u8> NRES_W<'a, O> {
    #[doc = "Only single result value will be returned. After a single read RRDY will be automatically cleared"]
    #[inline(always)]
    pub fn num1(self) -> &'a mut W {
        self.variant(NRES_A::Num1)
    }
    #[doc = "Two return reads need to be performed. After two reads RRDY will be automatically cleared"]
    #[inline(always)]
    pub fn num2(self) -> &'a mut W {
        self.variant(NRES_A::Num2)
    }
}
#[doc = "Field `NARGS` reader - Number of arguments expected by the WDATA register"]
pub type NARGS_R = crate::BitReader<NARGS_A>;
#[doc = "Number of arguments expected by the WDATA register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NARGS_A {
    #[doc = "0: Only single argument write is needed for next calculation"]
    Num1 = 0,
    #[doc = "1: Two argument writes need to be performed for next calculation"]
    Num2 = 1,
}
impl From<NARGS_A> for bool {
    #[inline(always)]
    fn from(variant: NARGS_A) -> Self {
        variant as u8 != 0
    }
}
impl NARGS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NARGS_A {
        match self.bits {
            false => NARGS_A::Num1,
            true => NARGS_A::Num2,
        }
    }
    #[doc = "Checks if the value of the field is `Num1`"]
    #[inline(always)]
    pub fn is_num1(&self) -> bool {
        *self == NARGS_A::Num1
    }
    #[doc = "Checks if the value of the field is `Num2`"]
    #[inline(always)]
    pub fn is_num2(&self) -> bool {
        *self == NARGS_A::Num2
    }
}
#[doc = "Field `NARGS` writer - Number of arguments expected by the WDATA register"]
pub type NARGS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, NARGS_A, O>;
impl<'a, const O: u8> NARGS_W<'a, O> {
    #[doc = "Only single argument write is needed for next calculation"]
    #[inline(always)]
    pub fn num1(self) -> &'a mut W {
        self.variant(NARGS_A::Num1)
    }
    #[doc = "Two argument writes need to be performed for next calculation"]
    #[inline(always)]
    pub fn num2(self) -> &'a mut W {
        self.variant(NARGS_A::Num2)
    }
}
#[doc = "Field `RESSIZE` reader - Width of output data"]
pub type RESSIZE_R = crate::BitReader<RESSIZE_A>;
#[doc = "Width of output data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESSIZE_A {
    #[doc = "0: Use 32 bit output values"]
    Bits32 = 0,
    #[doc = "1: Use 16 bit output values"]
    Bits16 = 1,
}
impl From<RESSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: RESSIZE_A) -> Self {
        variant as u8 != 0
    }
}
impl RESSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESSIZE_A {
        match self.bits {
            false => RESSIZE_A::Bits32,
            true => RESSIZE_A::Bits16,
        }
    }
    #[doc = "Checks if the value of the field is `Bits32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == RESSIZE_A::Bits32
    }
    #[doc = "Checks if the value of the field is `Bits16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == RESSIZE_A::Bits16
    }
}
#[doc = "Field `RESSIZE` writer - Width of output data"]
pub type RESSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RESSIZE_A, O>;
impl<'a, const O: u8> RESSIZE_W<'a, O> {
    #[doc = "Use 32 bit output values"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(RESSIZE_A::Bits32)
    }
    #[doc = "Use 16 bit output values"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(RESSIZE_A::Bits16)
    }
}
#[doc = "Field `ARGSIZE` reader - Width of input data"]
pub type ARGSIZE_R = crate::BitReader<ARGSIZE_A>;
#[doc = "Width of input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARGSIZE_A {
    #[doc = "0: Use 32 bit input values"]
    Bits32 = 0,
    #[doc = "1: Use 16 bit input values"]
    Bits16 = 1,
}
impl From<ARGSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: ARGSIZE_A) -> Self {
        variant as u8 != 0
    }
}
impl ARGSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARGSIZE_A {
        match self.bits {
            false => ARGSIZE_A::Bits32,
            true => ARGSIZE_A::Bits16,
        }
    }
    #[doc = "Checks if the value of the field is `Bits32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == ARGSIZE_A::Bits32
    }
    #[doc = "Checks if the value of the field is `Bits16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == ARGSIZE_A::Bits16
    }
}
#[doc = "Field `ARGSIZE` writer - Width of input data"]
pub type ARGSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, ARGSIZE_A, O>;
impl<'a, const O: u8> ARGSIZE_W<'a, O> {
    #[doc = "Use 32 bit input values"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(ARGSIZE_A::Bits32)
    }
    #[doc = "Use 16 bit input values"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(ARGSIZE_A::Bits16)
    }
}
#[doc = "Field `RRDY` reader - Result ready flag"]
pub type RRDY_R = crate::BitReader<RRDYR_A>;
#[doc = "Result ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRDYR_A {
    #[doc = "0: Results from computation are not read"]
    NotReady = 0,
    #[doc = "1: Results are ready, this flag will be automatically cleared once value is read"]
    Ready = 1,
}
impl From<RRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: RRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl RRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRDYR_A {
        match self.bits {
            false => RRDYR_A::NotReady,
            true => RRDYR_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == RRDYR_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == RRDYR_A::Ready
    }
}
#[doc = "Field `RRDY` writer - Result ready flag"]
pub type RRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RRDYR_A, O>;
impl<'a, const O: u8> RRDY_W<'a, O> {
    #[doc = "Results from computation are not read"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(RRDYR_A::NotReady)
    }
    #[doc = "Results are ready, this flag will be automatically cleared once value is read"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(RRDYR_A::Ready)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)"]
    #[inline(always)]
    pub fn precision(&self) -> PRECISION_R {
        PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Scaling factor (2^-n for arguments, 2^n for results)"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Enable interrupt"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable DMA wread channel"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable DMA write channel"]
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Number of results in the RDATA register"]
    #[inline(always)]
    pub fn nres(&self) -> NRES_R {
        NRES_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Number of arguments expected by the WDATA register"]
    #[inline(always)]
    pub fn nargs(&self) -> NARGS_R {
        NARGS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Width of output data"]
    #[inline(always)]
    pub fn ressize(&self) -> RESSIZE_R {
        RESSIZE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Width of input data"]
    #[inline(always)]
    pub fn argsize(&self) -> ARGSIZE_R {
        ARGSIZE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - Result ready flag"]
    #[inline(always)]
    pub fn rrdy(&self) -> RRDY_R {
        RRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<0> {
        FUNC_W::new(self)
    }
    #[doc = "Bits 4:7 - Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4)"]
    #[inline(always)]
    #[must_use]
    pub fn precision(&mut self) -> PRECISION_W<4> {
        PRECISION_W::new(self)
    }
    #[doc = "Bits 8:10 - Scaling factor (2^-n for arguments, 2^n for results)"]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<8> {
        SCALE_W::new(self)
    }
    #[doc = "Bit 16 - Enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IEN_W<16> {
        IEN_W::new(self)
    }
    #[doc = "Bit 17 - Enable DMA wread channel"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<17> {
        DMAREN_W::new(self)
    }
    #[doc = "Bit 18 - Enable DMA write channel"]
    #[inline(always)]
    #[must_use]
    pub fn dmawen(&mut self) -> DMAWEN_W<18> {
        DMAWEN_W::new(self)
    }
    #[doc = "Bit 19 - Number of results in the RDATA register"]
    #[inline(always)]
    #[must_use]
    pub fn nres(&mut self) -> NRES_W<19> {
        NRES_W::new(self)
    }
    #[doc = "Bit 20 - Number of arguments expected by the WDATA register"]
    #[inline(always)]
    #[must_use]
    pub fn nargs(&mut self) -> NARGS_W<20> {
        NARGS_W::new(self)
    }
    #[doc = "Bit 21 - Width of output data"]
    #[inline(always)]
    #[must_use]
    pub fn ressize(&mut self) -> RESSIZE_W<21> {
        RESSIZE_W::new(self)
    }
    #[doc = "Bit 22 - Width of input data"]
    #[inline(always)]
    #[must_use]
    pub fn argsize(&mut self) -> ARGSIZE_W<22> {
        ARGSIZE_W::new(self)
    }
    #[doc = "Bit 31 - Result ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn rrdy(&mut self) -> RRDY_W<31> {
        RRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0x50"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x50;
}
