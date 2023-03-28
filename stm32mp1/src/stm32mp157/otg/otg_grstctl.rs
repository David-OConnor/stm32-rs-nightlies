#[doc = "Register `OTG_GRSTCTL` reader"]
pub struct R(crate::R<OTG_GRSTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_GRSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_GRSTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_GRSTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_GRSTCTL` writer"]
pub struct W(crate::W<OTG_GRSTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_GRSTCTL_SPEC>;
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
impl From<crate::W<OTG_GRSTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_GRSTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSRST` reader - CSRST"]
pub type CSRST_R = crate::BitReader<bool>;
#[doc = "Field `CSRST` writer - CSRST"]
pub type CSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GRSTCTL_SPEC, bool, O>;
#[doc = "Field `PSRST` reader - PSRST"]
pub type PSRST_R = crate::BitReader<bool>;
#[doc = "Field `PSRST` writer - PSRST"]
pub type PSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GRSTCTL_SPEC, bool, O>;
#[doc = "Field `RXFFLSH` reader - RXFFLSH"]
pub type RXFFLSH_R = crate::BitReader<bool>;
#[doc = "Field `RXFFLSH` writer - RXFFLSH"]
pub type RXFFLSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GRSTCTL_SPEC, bool, O>;
#[doc = "Field `TXFFLSH` reader - TXFFLSH"]
pub type TXFFLSH_R = crate::BitReader<bool>;
#[doc = "Field `TXFFLSH` writer - TXFFLSH"]
pub type TXFFLSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GRSTCTL_SPEC, bool, O>;
#[doc = "Field `TXFNUM` reader - TXFNUM"]
pub type TXFNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFNUM` writer - TXFNUM"]
pub type TXFNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_GRSTCTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DMAREQ` reader - DMAREQ"]
pub type DMAREQ_R = crate::BitReader<bool>;
#[doc = "Field `AHBIDL` reader - AHBIDL"]
pub type AHBIDL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - CSRST"]
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PSRST"]
    #[inline(always)]
    pub fn psrst(&self) -> PSRST_R {
        PSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFFLSH"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXFFLSH"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TXFNUM"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - DMAREQ"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AHBIDL"]
    #[inline(always)]
    pub fn ahbidl(&self) -> AHBIDL_R {
        AHBIDL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSRST"]
    #[inline(always)]
    #[must_use]
    pub fn csrst(&mut self) -> CSRST_W<0> {
        CSRST_W::new(self)
    }
    #[doc = "Bit 1 - PSRST"]
    #[inline(always)]
    #[must_use]
    pub fn psrst(&mut self) -> PSRST_W<1> {
        PSRST_W::new(self)
    }
    #[doc = "Bit 4 - RXFFLSH"]
    #[inline(always)]
    #[must_use]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<4> {
        RXFFLSH_W::new(self)
    }
    #[doc = "Bit 5 - TXFFLSH"]
    #[inline(always)]
    #[must_use]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<5> {
        TXFFLSH_W::new(self)
    }
    #[doc = "Bits 6:10 - TXFNUM"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<6> {
        TXFNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The application uses this register to reset various hardware features inside the core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_grstctl](index.html) module"]
pub struct OTG_GRSTCTL_SPEC;
impl crate::RegisterSpec for OTG_GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_grstctl::R](R) reader structure"]
impl crate::Readable for OTG_GRSTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_grstctl::W](W) writer structure"]
impl crate::Writable for OTG_GRSTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_GRSTCTL to value 0x8000_0000"]
impl crate::Resettable for OTG_GRSTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
