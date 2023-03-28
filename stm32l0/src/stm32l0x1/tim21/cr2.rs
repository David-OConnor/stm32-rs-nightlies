#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMS` reader - Master mode selection"]
pub type MMS_R = crate::FieldReader<u8, MMS_A>;
#[doc = "Master mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS_A {
    #[doc = "0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO)"]
    Reset = 0,
    #[doc = "1: Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO)"]
    Enable = 1,
    #[doc = "2: Update - The update event is selected as trigger output (TRGO)"]
    Update = 2,
    #[doc = "3: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred"]
    ComparePulse = 3,
    #[doc = "4: OC1REF signal is used as trigger output (TRGO)"]
    Oc1ref = 4,
    #[doc = "5: OC2REF signal is used as trigger output (TRGO)"]
    Oc2ref = 5,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
impl MMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MMS_A> {
        match self.bits {
            0 => Some(MMS_A::Reset),
            1 => Some(MMS_A::Enable),
            2 => Some(MMS_A::Update),
            3 => Some(MMS_A::ComparePulse),
            4 => Some(MMS_A::Oc1ref),
            5 => Some(MMS_A::Oc2ref),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS_A::Reset
    }
    #[doc = "Checks if the value of the field is `Enable`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS_A::Enable
    }
    #[doc = "Checks if the value of the field is `Update`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS_A::Update
    }
    #[doc = "Checks if the value of the field is `ComparePulse`"]
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMS_A::ComparePulse
    }
    #[doc = "Checks if the value of the field is `Oc1ref`"]
    #[inline(always)]
    pub fn is_oc1ref(&self) -> bool {
        *self == MMS_A::Oc1ref
    }
    #[doc = "Checks if the value of the field is `Oc2ref`"]
    #[inline(always)]
    pub fn is_oc2ref(&self) -> bool {
        *self == MMS_A::Oc2ref
    }
}
#[doc = "Field `MMS` writer - Master mode selection"]
pub type MMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, MMS_A, 3, O>;
impl<'a, const O: u8> MMS_W<'a, O> {
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMS_A::Reset)
    }
    #[doc = "Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMS_A::Enable)
    }
    #[doc = "Update - The update event is selected as trigger output (TRGO)"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMS_A::Update)
    }
    #[doc = "Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred"]
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut W {
        self.variant(MMS_A::ComparePulse)
    }
    #[doc = "OC1REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn oc1ref(self) -> &'a mut W {
        self.variant(MMS_A::Oc1ref)
    }
    #[doc = "OC2REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn oc2ref(self) -> &'a mut W {
        self.variant(MMS_A::Oc2ref)
    }
}
impl R {
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<4> {
        MMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
