#[doc = "Register `OTG_DCTL` reader"]
pub struct R(crate::R<OTG_DCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DCTL` writer"]
pub struct W(crate::W<OTG_DCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DCTL_SPEC>;
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
impl From<crate::W<OTG_DCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWUSIG` reader - RWUSIG"]
pub type RWUSIG_R = crate::BitReader<bool>;
#[doc = "Field `RWUSIG` writer - RWUSIG"]
pub type RWUSIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DCTL_SPEC, bool, O>;
#[doc = "Field `SDIS` reader - SDIS"]
pub type SDIS_R = crate::BitReader<bool>;
#[doc = "Field `SDIS` writer - SDIS"]
pub type SDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DCTL_SPEC, bool, O>;
#[doc = "Field `GINSTS` reader - GINSTS"]
pub type GINSTS_R = crate::BitReader<bool>;
#[doc = "Field `GONSTS` reader - GONSTS"]
pub type GONSTS_R = crate::BitReader<bool>;
#[doc = "Field `TCTL` reader - TCTL"]
pub type TCTL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCTL` writer - TCTL"]
pub type TCTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_DCTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SGINAK` writer - SGINAK"]
pub type SGINAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DCTL_SPEC, bool, O>;
#[doc = "Field `CGINAK` writer - CGINAK"]
pub type CGINAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DCTL_SPEC, bool, O>;
#[doc = "Field `SGONAK` writer - SGONAK"]
pub type SGONAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DCTL_SPEC, bool, O>;
#[doc = "Field `CGONAK` writer - CGONAK"]
pub type CGONAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DCTL_SPEC, bool, O>;
#[doc = "Field `POPRGDNE` reader - POPRGDNE"]
pub type POPRGDNE_R = crate::BitReader<bool>;
#[doc = "Field `POPRGDNE` writer - POPRGDNE"]
pub type POPRGDNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DCTL_SPEC, bool, O>;
#[doc = "Field `DSBESLRJCT` reader - DSBESLRJCT"]
pub type DSBESLRJCT_R = crate::BitReader<bool>;
#[doc = "Field `DSBESLRJCT` writer - DSBESLRJCT"]
pub type DSBESLRJCT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RWUSIG"]
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDIS"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GINSTS"]
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GONSTS"]
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - TCTL"]
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - POPRGDNE"]
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 18 - DSBESLRJCT"]
    #[inline(always)]
    pub fn dsbeslrjct(&self) -> DSBESLRJCT_R {
        DSBESLRJCT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RWUSIG"]
    #[inline(always)]
    #[must_use]
    pub fn rwusig(&mut self) -> RWUSIG_W<0> {
        RWUSIG_W::new(self)
    }
    #[doc = "Bit 1 - SDIS"]
    #[inline(always)]
    #[must_use]
    pub fn sdis(&mut self) -> SDIS_W<1> {
        SDIS_W::new(self)
    }
    #[doc = "Bits 4:6 - TCTL"]
    #[inline(always)]
    #[must_use]
    pub fn tctl(&mut self) -> TCTL_W<4> {
        TCTL_W::new(self)
    }
    #[doc = "Bit 7 - SGINAK"]
    #[inline(always)]
    #[must_use]
    pub fn sginak(&mut self) -> SGINAK_W<7> {
        SGINAK_W::new(self)
    }
    #[doc = "Bit 8 - CGINAK"]
    #[inline(always)]
    #[must_use]
    pub fn cginak(&mut self) -> CGINAK_W<8> {
        CGINAK_W::new(self)
    }
    #[doc = "Bit 9 - SGONAK"]
    #[inline(always)]
    #[must_use]
    pub fn sgonak(&mut self) -> SGONAK_W<9> {
        SGONAK_W::new(self)
    }
    #[doc = "Bit 10 - CGONAK"]
    #[inline(always)]
    #[must_use]
    pub fn cgonak(&mut self) -> CGONAK_W<10> {
        CGONAK_W::new(self)
    }
    #[doc = "Bit 11 - POPRGDNE"]
    #[inline(always)]
    #[must_use]
    pub fn poprgdne(&mut self) -> POPRGDNE_W<11> {
        POPRGDNE_W::new(self)
    }
    #[doc = "Bit 18 - DSBESLRJCT"]
    #[inline(always)]
    #[must_use]
    pub fn dsbeslrjct(&mut self) -> DSBESLRJCT_W<18> {
        DSBESLRJCT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG device control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dctl](index.html) module"]
pub struct OTG_DCTL_SPEC;
impl crate::RegisterSpec for OTG_DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_dctl::R](R) reader structure"]
impl crate::Readable for OTG_DCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_dctl::W](W) writer structure"]
impl crate::Writable for OTG_DCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_DCTL to value 0x02"]
impl crate::Resettable for OTG_DCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
