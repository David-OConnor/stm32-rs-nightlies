#[doc = "Register `PDCRH` reader"]
pub struct R(crate::R<PDCRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDCRH` writer"]
pub struct W(crate::W<PDCRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRH_SPEC>;
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
impl From<crate::W<PDCRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD3` reader - pull-down"]
pub type PD3_R = crate::BitReader<PD3_A>;
#[doc = "pull-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD3_A {
    #[doc = "0: Disable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Disabled = 0,
    #[doc = "1: Enable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    Enabled = 1,
}
impl From<PD3_A> for bool {
    #[inline(always)]
    fn from(variant: PD3_A) -> Self {
        variant as u8 != 0
    }
}
impl PD3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD3_A {
        match self.bits {
            false => PD3_A::Disabled,
            true => PD3_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PD3_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD3_A::Enabled
    }
}
#[doc = "Field `PD3` writer - pull-down"]
pub type PD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRH_SPEC, PD3_A, O>;
impl<'a, const O: u8> PD3_W<'a, O> {
    #[doc = "Disable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD3_A::Disabled)
    }
    #[doc = "Enable the pull-down on PH\\[y\\]
when both APC bits are set in PWR control register 3 (PWR_CR3)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD3_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 3 - pull-down"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - pull-down"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<3> {
        PD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port H pull-down control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdcrh](index.html) module"]
pub struct PDCRH_SPEC;
impl crate::RegisterSpec for PDCRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdcrh::R](R) reader structure"]
impl crate::Readable for PDCRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdcrh::W](W) writer structure"]
impl crate::Writable for PDCRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDCRH to value 0"]
impl crate::Resettable for PDCRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
