#[doc = "Register `SMPR1` reader"]
pub struct R(crate::R<SMPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR1` writer"]
pub struct W(crate::W<SMPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR1_SPEC>;
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
impl From<crate::W<SMPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP0` reader - ADC channel 0 sampling time selection"]
pub type SMP0_R = crate::FieldReader<u8, SMP0_A>;
#[doc = "ADC channel 0 sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP0_A {
    #[doc = "0: 1.5 ADC clock cycles"]
    Cycles15 = 0,
    #[doc = "1: 2.5 ADC clock cycles"]
    Cycles25 = 1,
    #[doc = "2: 8.5 ADC clock cycles"]
    Cycles85 = 2,
    #[doc = "3: 16.5 ADC clock cycles"]
    Cycles165 = 3,
    #[doc = "4: 32.5 ADC clock cycles"]
    Cycles325 = 4,
    #[doc = "5: 64.5 ADC clock cycles"]
    Cycles645 = 5,
    #[doc = "6: 387.5 ADC clock cycles"]
    Cycles3875 = 6,
    #[doc = "7: 810.5 ADC clock cycles"]
    Cycles8105 = 7,
}
impl From<SMP0_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP0_A) -> Self {
        variant as _
    }
}
impl SMP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP0_A {
        match self.bits {
            0 => SMP0_A::Cycles15,
            1 => SMP0_A::Cycles25,
            2 => SMP0_A::Cycles85,
            3 => SMP0_A::Cycles165,
            4 => SMP0_A::Cycles325,
            5 => SMP0_A::Cycles645,
            6 => SMP0_A::Cycles3875,
            7 => SMP0_A::Cycles8105,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Cycles15`"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP0_A::Cycles15
    }
    #[doc = "Checks if the value of the field is `Cycles25`"]
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        *self == SMP0_A::Cycles25
    }
    #[doc = "Checks if the value of the field is `Cycles85`"]
    #[inline(always)]
    pub fn is_cycles8_5(&self) -> bool {
        *self == SMP0_A::Cycles85
    }
    #[doc = "Checks if the value of the field is `Cycles165`"]
    #[inline(always)]
    pub fn is_cycles16_5(&self) -> bool {
        *self == SMP0_A::Cycles165
    }
    #[doc = "Checks if the value of the field is `Cycles325`"]
    #[inline(always)]
    pub fn is_cycles32_5(&self) -> bool {
        *self == SMP0_A::Cycles325
    }
    #[doc = "Checks if the value of the field is `Cycles645`"]
    #[inline(always)]
    pub fn is_cycles64_5(&self) -> bool {
        *self == SMP0_A::Cycles645
    }
    #[doc = "Checks if the value of the field is `Cycles3875`"]
    #[inline(always)]
    pub fn is_cycles387_5(&self) -> bool {
        *self == SMP0_A::Cycles3875
    }
    #[doc = "Checks if the value of the field is `Cycles8105`"]
    #[inline(always)]
    pub fn is_cycles810_5(&self) -> bool {
        *self == SMP0_A::Cycles8105
    }
}
#[doc = "Field `SMP0` writer - ADC channel 0 sampling time selection"]
pub type SMP0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMPR1_SPEC, u8, SMP0_A, 3, O>;
impl<'a, const O: u8> SMP0_W<'a, O> {
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles15)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles25)
    }
    #[doc = "8.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles8_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles85)
    }
    #[doc = "16.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles16_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles165)
    }
    #[doc = "32.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles32_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles325)
    }
    #[doc = "64.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles64_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles645)
    }
    #[doc = "387.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles387_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles3875)
    }
    #[doc = "810.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles810_5(self) -> &'a mut W {
        self.variant(SMP0_A::Cycles8105)
    }
}
#[doc = "Field `SMP1` reader - ADC channel 1 sampling time selection"]
pub use SMP0_R as SMP1_R;
#[doc = "Field `SMP2` reader - ADC channel 2 sampling time selection"]
pub use SMP0_R as SMP2_R;
#[doc = "Field `SMP3` reader - ADC channel 3 sampling time selection"]
pub use SMP0_R as SMP3_R;
#[doc = "Field `SMP4` reader - ADC channel 4 sampling time selection"]
pub use SMP0_R as SMP4_R;
#[doc = "Field `SMP5` reader - ADC channel 5 sampling time selection"]
pub use SMP0_R as SMP5_R;
#[doc = "Field `SMP6` reader - ADC channel 6 sampling time selection"]
pub use SMP0_R as SMP6_R;
#[doc = "Field `SMP7` reader - ADC channel 7 sampling time selection"]
pub use SMP0_R as SMP7_R;
#[doc = "Field `SMP8` reader - ADC channel 8 sampling time selection"]
pub use SMP0_R as SMP8_R;
#[doc = "Field `SMP9` reader - ADC channel 9 sampling time selection"]
pub use SMP0_R as SMP9_R;
#[doc = "Field `SMP1` writer - ADC channel 1 sampling time selection"]
pub use SMP0_W as SMP1_W;
#[doc = "Field `SMP2` writer - ADC channel 2 sampling time selection"]
pub use SMP0_W as SMP2_W;
#[doc = "Field `SMP3` writer - ADC channel 3 sampling time selection"]
pub use SMP0_W as SMP3_W;
#[doc = "Field `SMP4` writer - ADC channel 4 sampling time selection"]
pub use SMP0_W as SMP4_W;
#[doc = "Field `SMP5` writer - ADC channel 5 sampling time selection"]
pub use SMP0_W as SMP5_W;
#[doc = "Field `SMP6` writer - ADC channel 6 sampling time selection"]
pub use SMP0_W as SMP6_W;
#[doc = "Field `SMP7` writer - ADC channel 7 sampling time selection"]
pub use SMP0_W as SMP7_W;
#[doc = "Field `SMP8` writer - ADC channel 8 sampling time selection"]
pub use SMP0_W as SMP8_W;
#[doc = "Field `SMP9` writer - ADC channel 9 sampling time selection"]
pub use SMP0_W as SMP9_W;
impl R {
    #[doc = "Bits 0:2 - ADC channel 0 sampling time selection"]
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - ADC channel 1 sampling time selection"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - ADC channel 2 sampling time selection"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - ADC channel 3 sampling time selection"]
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - ADC channel 4 sampling time selection"]
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - ADC channel 5 sampling time selection"]
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - ADC channel 6 sampling time selection"]
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - ADC channel 7 sampling time selection"]
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - ADC channel 8 sampling time selection"]
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - ADC channel 9 sampling time selection"]
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC channel 0 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp0(&mut self) -> SMP0_W<0> {
        SMP0_W::new(self)
    }
    #[doc = "Bits 3:5 - ADC channel 1 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<3> {
        SMP1_W::new(self)
    }
    #[doc = "Bits 6:8 - ADC channel 2 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<6> {
        SMP2_W::new(self)
    }
    #[doc = "Bits 9:11 - ADC channel 3 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp3(&mut self) -> SMP3_W<9> {
        SMP3_W::new(self)
    }
    #[doc = "Bits 12:14 - ADC channel 4 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp4(&mut self) -> SMP4_W<12> {
        SMP4_W::new(self)
    }
    #[doc = "Bits 15:17 - ADC channel 5 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp5(&mut self) -> SMP5_W<15> {
        SMP5_W::new(self)
    }
    #[doc = "Bits 18:20 - ADC channel 6 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp6(&mut self) -> SMP6_W<18> {
        SMP6_W::new(self)
    }
    #[doc = "Bits 21:23 - ADC channel 7 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp7(&mut self) -> SMP7_W<21> {
        SMP7_W::new(self)
    }
    #[doc = "Bits 24:26 - ADC channel 8 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp8(&mut self) -> SMP8_W<24> {
        SMP8_W::new(self)
    }
    #[doc = "Bits 27:29 - ADC channel 9 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp9(&mut self) -> SMP9_W<27> {
        SMP9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC sampling time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr1](index.html) module"]
pub struct SMPR1_SPEC;
impl crate::RegisterSpec for SMPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr1::R](R) reader structure"]
impl crate::Readable for SMPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr1::W](W) writer structure"]
impl crate::Writable for SMPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for SMPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
