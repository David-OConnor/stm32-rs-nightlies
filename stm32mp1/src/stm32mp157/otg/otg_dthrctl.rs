#[doc = "Register `OTG_DTHRCTL` reader"]
pub struct R(crate::R<OTG_DTHRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DTHRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DTHRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DTHRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DTHRCTL` writer"]
pub struct W(crate::W<OTG_DTHRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DTHRCTL_SPEC>;
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
impl From<crate::W<OTG_DTHRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DTHRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONISOTHREN` reader - NONISOTHREN"]
pub type NONISOTHREN_R = crate::BitReader<bool>;
#[doc = "Field `NONISOTHREN` writer - NONISOTHREN"]
pub type NONISOTHREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DTHRCTL_SPEC, bool, O>;
#[doc = "Field `ISOTHREN` reader - ISOTHREN"]
pub type ISOTHREN_R = crate::BitReader<bool>;
#[doc = "Field `ISOTHREN` writer - ISOTHREN"]
pub type ISOTHREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DTHRCTL_SPEC, bool, O>;
#[doc = "Field `TXTHRLEN` reader - TXTHRLEN"]
pub type TXTHRLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXTHRLEN` writer - TXTHRLEN"]
pub type TXTHRLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_DTHRCTL_SPEC, u16, u16, 9, O>;
#[doc = "Field `RXTHREN` reader - RXTHREN"]
pub type RXTHREN_R = crate::BitReader<bool>;
#[doc = "Field `RXTHREN` writer - RXTHREN"]
pub type RXTHREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DTHRCTL_SPEC, bool, O>;
#[doc = "Field `RXTHRLEN` reader - RXTHRLEN"]
pub type RXTHRLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXTHRLEN` writer - RXTHRLEN"]
pub type RXTHRLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_DTHRCTL_SPEC, u16, u16, 9, O>;
#[doc = "Field `ARPEN` reader - ARPEN"]
pub type ARPEN_R = crate::BitReader<bool>;
#[doc = "Field `ARPEN` writer - ARPEN"]
pub type ARPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DTHRCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - NONISOTHREN"]
    #[inline(always)]
    pub fn nonisothren(&self) -> NONISOTHREN_R {
        NONISOTHREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ISOTHREN"]
    #[inline(always)]
    pub fn isothren(&self) -> ISOTHREN_R {
        ISOTHREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - TXTHRLEN"]
    #[inline(always)]
    pub fn txthrlen(&self) -> TXTHRLEN_R {
        TXTHRLEN_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - RXTHREN"]
    #[inline(always)]
    pub fn rxthren(&self) -> RXTHREN_R {
        RXTHREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:25 - RXTHRLEN"]
    #[inline(always)]
    pub fn rxthrlen(&self) -> RXTHRLEN_R {
        RXTHRLEN_R::new(((self.bits >> 17) & 0x01ff) as u16)
    }
    #[doc = "Bit 27 - ARPEN"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NONISOTHREN"]
    #[inline(always)]
    #[must_use]
    pub fn nonisothren(&mut self) -> NONISOTHREN_W<0> {
        NONISOTHREN_W::new(self)
    }
    #[doc = "Bit 1 - ISOTHREN"]
    #[inline(always)]
    #[must_use]
    pub fn isothren(&mut self) -> ISOTHREN_W<1> {
        ISOTHREN_W::new(self)
    }
    #[doc = "Bits 2:10 - TXTHRLEN"]
    #[inline(always)]
    #[must_use]
    pub fn txthrlen(&mut self) -> TXTHRLEN_W<2> {
        TXTHRLEN_W::new(self)
    }
    #[doc = "Bit 16 - RXTHREN"]
    #[inline(always)]
    #[must_use]
    pub fn rxthren(&mut self) -> RXTHREN_W<16> {
        RXTHREN_W::new(self)
    }
    #[doc = "Bits 17:25 - RXTHRLEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxthrlen(&mut self) -> RXTHRLEN_W<17> {
        RXTHRLEN_W::new(self)
    }
    #[doc = "Bit 27 - ARPEN"]
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ARPEN_W<27> {
        ARPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG device threshold control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dthrctl](index.html) module"]
pub struct OTG_DTHRCTL_SPEC;
impl crate::RegisterSpec for OTG_DTHRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_dthrctl::R](R) reader structure"]
impl crate::Readable for OTG_DTHRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_dthrctl::W](W) writer structure"]
impl crate::Writable for OTG_DTHRCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_DTHRCTL to value 0"]
impl crate::Resettable for OTG_DTHRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
