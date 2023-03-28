#[doc = "Register `SMPR` reader"]
pub struct R(crate::R<SMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR` writer"]
pub struct W(crate::W<SMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR_SPEC>;
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
impl From<crate::W<SMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP` reader - Sampling time selection"]
pub type SMP_R = crate::FieldReader<u8, SMP_A>;
#[doc = "Sampling time selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP_A {
    #[doc = "0: 1.5 ADC clock cycles"]
    Cycles15 = 0,
    #[doc = "1: 3.5 ADC clock cycles"]
    Cycles35 = 1,
    #[doc = "2: 7.5 ADC clock cycles"]
    Cycles75 = 2,
    #[doc = "3: 12.5 ADC clock cycles"]
    Cycles125 = 3,
    #[doc = "4: 19.5 ADC clock cycles"]
    Cycles195 = 4,
    #[doc = "5: 39.5 ADC clock cycles"]
    Cycles395 = 5,
    #[doc = "6: 79.5 ADC clock cycles"]
    Cycles795 = 6,
    #[doc = "7: 160.5 ADC clock cycles"]
    Cycles1605 = 7,
}
impl From<SMP_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP_A) -> Self {
        variant as _
    }
}
impl SMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP_A {
        match self.bits {
            0 => SMP_A::Cycles15,
            1 => SMP_A::Cycles35,
            2 => SMP_A::Cycles75,
            3 => SMP_A::Cycles125,
            4 => SMP_A::Cycles195,
            5 => SMP_A::Cycles395,
            6 => SMP_A::Cycles795,
            7 => SMP_A::Cycles1605,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Cycles15`"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP_A::Cycles15
    }
    #[doc = "Checks if the value of the field is `Cycles35`"]
    #[inline(always)]
    pub fn is_cycles3_5(&self) -> bool {
        *self == SMP_A::Cycles35
    }
    #[doc = "Checks if the value of the field is `Cycles75`"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP_A::Cycles75
    }
    #[doc = "Checks if the value of the field is `Cycles125`"]
    #[inline(always)]
    pub fn is_cycles12_5(&self) -> bool {
        *self == SMP_A::Cycles125
    }
    #[doc = "Checks if the value of the field is `Cycles195`"]
    #[inline(always)]
    pub fn is_cycles19_5(&self) -> bool {
        *self == SMP_A::Cycles195
    }
    #[doc = "Checks if the value of the field is `Cycles395`"]
    #[inline(always)]
    pub fn is_cycles39_5(&self) -> bool {
        *self == SMP_A::Cycles395
    }
    #[doc = "Checks if the value of the field is `Cycles795`"]
    #[inline(always)]
    pub fn is_cycles79_5(&self) -> bool {
        *self == SMP_A::Cycles795
    }
    #[doc = "Checks if the value of the field is `Cycles1605`"]
    #[inline(always)]
    pub fn is_cycles160_5(&self) -> bool {
        *self == SMP_A::Cycles1605
    }
}
#[doc = "Field `SMP` writer - Sampling time selection"]
pub type SMP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMPR_SPEC, u8, SMP_A, 3, O>;
impl<'a, const O: u8> SMP_W<'a, O> {
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles15)
    }
    #[doc = "3.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles3_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles35)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles75)
    }
    #[doc = "12.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles12_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles125)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles195)
    }
    #[doc = "39.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles39_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles395)
    }
    #[doc = "79.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles79_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles795)
    }
    #[doc = "160.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles160_5(self) -> &'a mut W {
        self.variant(SMP_A::Cycles1605)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp(&mut self) -> SMP_W<0> {
        SMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sampling time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr](index.html) module"]
pub struct SMPR_SPEC;
impl crate::RegisterSpec for SMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr::R](R) reader structure"]
impl crate::Readable for SMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr::W](W) writer structure"]
impl crate::Writable for SMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR to value 0"]
impl crate::Resettable for SMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
