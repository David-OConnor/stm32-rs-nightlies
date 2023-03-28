#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVR_IE` reader - Data buffer overrun/underrun interrupt enable"]
pub type OVR_IE_R = crate::BitReader<OVR_IE_A>;
#[doc = "Data buffer overrun/underrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_IE_A {
    #[doc = "0: No interrupt generation"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated if either an overrun or an underrun error occurred"]
    Enabled = 1,
}
impl From<OVR_IE_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_IE_A {
        match self.bits {
            false => OVR_IE_A::Disabled,
            true => OVR_IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_IE_A::Enabled
    }
}
#[doc = "Field `OVR_IE` writer - Data buffer overrun/underrun interrupt enable"]
pub type OVR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, OVR_IE_A, O>;
impl<'a, const O: u8> OVR_IE_W<'a, O> {
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVR_IE_A::Disabled)
    }
    #[doc = "An interrupt is generated if either an overrun or an underrun error occurred"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVR_IE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 1 - Data buffer overrun/underrun interrupt enable"]
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Data buffer overrun/underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr_ie(&mut self) -> OVR_IE_W<1> {
        OVR_IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PSSI interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
