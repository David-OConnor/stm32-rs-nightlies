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
#[doc = "Field `FUNC` reader - FUNC"]
pub type FUNC_R = crate::FieldReader<u8, FUNC_A>;
#[doc = "FUNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: Cosine funciton"]
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
#[doc = "Field `FUNC` writer - FUNC"]
pub type FUNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, FUNC_A, 4, O>;
impl<'a, const O: u8> FUNC_W<'a, O> {
    #[doc = "Cosine funciton"]
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
#[doc = "Field `PRECISION` reader - Precision (number of iterations/cycles) required"]
pub type PRECISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRECISION` writer - Precision (number of iterations/cycles) required"]
pub type PRECISION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SCALE` reader - Scaling factor (2^-n for arguments, 2^n for results)"]
pub type SCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCALE` writer - Scaling factor (2^-n for arguments, 2^n for results)"]
pub type SCALE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `IEN` reader - IEN"]
pub type IEN_R = crate::BitReader<IEN_A>;
#[doc = "IEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN_A {
    #[doc = "0: Disable interrupt request generation"]
    Disabled = 0,
    #[doc = "1: Enable intterrupt request generation"]
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
#[doc = "Field `IEN` writer - IEN"]
pub type IEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, IEN_A, O>;
impl<'a, const O: u8> IEN_W<'a, O> {
    #[doc = "Disable interrupt request generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IEN_A::Disabled)
    }
    #[doc = "Enable intterrupt request generation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IEN_A::Enabled)
    }
}
#[doc = "Field `DMAREN` reader - DMAREN"]
pub type DMAREN_R = crate::BitReader<DMAREN_A>;
#[doc = "DMAREN\n\nValue on reset: 0"]
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
#[doc = "Field `DMAREN` writer - DMAREN"]
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
#[doc = "Field `DMAWEN` reader - DMAWEN"]
pub type DMAWEN_R = crate::BitReader<DMAWEN_A>;
#[doc = "DMAWEN\n\nValue on reset: 0"]
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
#[doc = "Field `DMAWEN` writer - DMAWEN"]
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
#[doc = "Field `NRES` reader - NRES"]
pub type NRES_R = crate::BitReader<NRES_A>;
#[doc = "NRES\n\nValue on reset: 0"]
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
#[doc = "Field `NRES` writer - NRES"]
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
#[doc = "Field `NARGS` reader - NARGS"]
pub type NARGS_R = crate::BitReader<NARGS_A>;
#[doc = "NARGS\n\nValue on reset: 0"]
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
#[doc = "Field `NARGS` writer - NARGS"]
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
#[doc = "Field `RESSIZE` reader - RESSIZE"]
pub type RESSIZE_R = crate::BitReader<RESSIZE_A>;
#[doc = "RESSIZE\n\nValue on reset: 0"]
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
#[doc = "Field `RESSIZE` writer - RESSIZE"]
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
#[doc = "Field `ARGSIZE` reader - ARGSIZE"]
pub type ARGSIZE_R = crate::BitReader<ARGSIZE_A>;
#[doc = "ARGSIZE\n\nValue on reset: 0"]
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
#[doc = "Field `ARGSIZE` writer - ARGSIZE"]
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
#[doc = "Field `RRDY` reader - RRDY"]
pub type RRDY_R = crate::BitReader<RRDYR_A>;
#[doc = "RRDY\n\nValue on reset: 0"]
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
#[doc = "Field `RRDY` writer - RRDY"]
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
    #[doc = "Bits 0:3 - FUNC"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Precision (number of iterations/cycles) required"]
    #[inline(always)]
    pub fn precision(&self) -> PRECISION_R {
        PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Scaling factor (2^-n for arguments, 2^n for results)"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - IEN"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMAREN"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMAWEN"]
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NRES"]
    #[inline(always)]
    pub fn nres(&self) -> NRES_R {
        NRES_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - NARGS"]
    #[inline(always)]
    pub fn nargs(&self) -> NARGS_R {
        NARGS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RESSIZE"]
    #[inline(always)]
    pub fn ressize(&self) -> RESSIZE_R {
        RESSIZE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ARGSIZE"]
    #[inline(always)]
    pub fn argsize(&self) -> ARGSIZE_R {
        ARGSIZE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - RRDY"]
    #[inline(always)]
    pub fn rrdy(&self) -> RRDY_R {
        RRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FUNC"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<0> {
        FUNC_W::new(self)
    }
    #[doc = "Bits 4:7 - Precision (number of iterations/cycles) required"]
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
    #[doc = "Bit 16 - IEN"]
    #[inline(always)]
    #[must_use]
    pub fn ien(&mut self) -> IEN_W<16> {
        IEN_W::new(self)
    }
    #[doc = "Bit 17 - DMAREN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<17> {
        DMAREN_W::new(self)
    }
    #[doc = "Bit 18 - DMAWEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmawen(&mut self) -> DMAWEN_W<18> {
        DMAWEN_W::new(self)
    }
    #[doc = "Bit 19 - NRES"]
    #[inline(always)]
    #[must_use]
    pub fn nres(&mut self) -> NRES_W<19> {
        NRES_W::new(self)
    }
    #[doc = "Bit 20 - NARGS"]
    #[inline(always)]
    #[must_use]
    pub fn nargs(&mut self) -> NARGS_W<20> {
        NARGS_W::new(self)
    }
    #[doc = "Bit 21 - RESSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn ressize(&mut self) -> RESSIZE_W<21> {
        RESSIZE_W::new(self)
    }
    #[doc = "Bit 22 - ARGSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn argsize(&mut self) -> ARGSIZE_W<22> {
        ARGSIZE_W::new(self)
    }
    #[doc = "Bit 31 - RRDY"]
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
#[doc = "CORDIC Control Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
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
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
