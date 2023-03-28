#[doc = "Register `DDRPHYC_ZQ0CR0` reader"]
pub struct R(crate::R<DDRPHYC_ZQ0CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ZQ0CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ZQ0CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ZQ0CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_ZQ0CR0` writer"]
pub struct W(crate::W<DDRPHYC_ZQ0CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_ZQ0CR0_SPEC>;
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
impl From<crate::W<DDRPHYC_ZQ0CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_ZQ0CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ZDATA` reader - ZDATA"]
pub type ZDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ZDATA` writer - ZDATA"]
pub type ZDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRPHYC_ZQ0CR0_SPEC, u32, u32, 20, O>;
#[doc = "Field `ZDEN` reader - ZDEN"]
pub type ZDEN_R = crate::BitReader<bool>;
#[doc = "Field `ZDEN` writer - ZDEN"]
pub type ZDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ZQ0CR0_SPEC, bool, O>;
#[doc = "Field `ZCALBYP` reader - ZCALBYP"]
pub type ZCALBYP_R = crate::BitReader<bool>;
#[doc = "Field `ZCALBYP` writer - ZCALBYP"]
pub type ZCALBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ZQ0CR0_SPEC, bool, O>;
#[doc = "Field `ZCAL` reader - ZCAL"]
pub type ZCAL_R = crate::BitReader<bool>;
#[doc = "Field `ZCAL` writer - ZCAL"]
pub type ZCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ZQ0CR0_SPEC, bool, O>;
#[doc = "Field `ZQPD` reader - ZQPD"]
pub type ZQPD_R = crate::BitReader<bool>;
#[doc = "Field `ZQPD` writer - ZQPD"]
pub type ZQPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ZQ0CR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:19 - ZDATA"]
    #[inline(always)]
    pub fn zdata(&self) -> ZDATA_R {
        ZDATA_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28 - ZDEN"]
    #[inline(always)]
    pub fn zden(&self) -> ZDEN_R {
        ZDEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ZCALBYP"]
    #[inline(always)]
    pub fn zcalbyp(&self) -> ZCALBYP_R {
        ZCALBYP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ZCAL"]
    #[inline(always)]
    pub fn zcal(&self) -> ZCAL_R {
        ZCAL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ZQPD"]
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - ZDATA"]
    #[inline(always)]
    #[must_use]
    pub fn zdata(&mut self) -> ZDATA_W<0> {
        ZDATA_W::new(self)
    }
    #[doc = "Bit 28 - ZDEN"]
    #[inline(always)]
    #[must_use]
    pub fn zden(&mut self) -> ZDEN_W<28> {
        ZDEN_W::new(self)
    }
    #[doc = "Bit 29 - ZCALBYP"]
    #[inline(always)]
    #[must_use]
    pub fn zcalbyp(&mut self) -> ZCALBYP_W<29> {
        ZCALBYP_W::new(self)
    }
    #[doc = "Bit 30 - ZCAL"]
    #[inline(always)]
    #[must_use]
    pub fn zcal(&mut self) -> ZCAL_W<30> {
        ZCAL_W::new(self)
    }
    #[doc = "Bit 31 - ZQPD"]
    #[inline(always)]
    #[must_use]
    pub fn zqpd(&mut self) -> ZQPD_W<31> {
        ZQPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC ZQ0C register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0cr0](index.html) module"]
pub struct DDRPHYC_ZQ0CR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_ZQ0CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_zq0cr0::R](R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_zq0cr0::W](W) writer structure"]
impl crate::Writable for DDRPHYC_ZQ0CR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRPHYC_ZQ0CR0 to value 0x014a"]
impl crate::Resettable for DDRPHYC_ZQ0CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x014a;
}
