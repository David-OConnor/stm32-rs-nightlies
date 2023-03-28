#[doc = "Register `ETH_MACLTCR` reader"]
pub struct R(crate::R<ETH_MACLTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACLTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACLTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACLTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACLTCR` writer"]
pub struct W(crate::W<ETH_MACLTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACLTCR_SPEC>;
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
impl From<crate::W<ETH_MACLTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACLTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWT` reader - TWT"]
pub type TWT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TWT` writer - TWT"]
pub type TWT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACLTCR_SPEC, u16, u16, 16, O>;
#[doc = "Field `LST` reader - LST"]
pub type LST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LST` writer - LST"]
pub type LST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACLTCR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:15 - TWT"]
    #[inline(always)]
    pub fn twt(&self) -> TWT_R {
        TWT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - LST"]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TWT"]
    #[inline(always)]
    #[must_use]
    pub fn twt(&mut self) -> TWT_W<0> {
        TWT_W::new(self)
    }
    #[doc = "Bits 16:25 - LST"]
    #[inline(always)]
    #[must_use]
    pub fn lst(&mut self) -> LST_W<16> {
        LST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The LPI Timers Control register controls the timeout values in the LPI states. It specifies the time for which the MAC transmits the LPI pattern and also the time for which the MAC waits before resuming the normal transmission.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macltcr](index.html) module"]
pub struct ETH_MACLTCR_SPEC;
impl crate::RegisterSpec for ETH_MACLTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macltcr::R](R) reader structure"]
impl crate::Readable for ETH_MACLTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macltcr::W](W) writer structure"]
impl crate::Writable for ETH_MACLTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACLTCR to value 0x03e8_0000"]
impl crate::Resettable for ETH_MACLTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03e8_0000;
}
