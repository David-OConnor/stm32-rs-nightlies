#[doc = "Register `DFSDM_CHCFG2R1` reader"]
pub struct R(crate::R<DFSDM_CHCFG2R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_CHCFG2R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_CHCFG2R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_CHCFG2R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_CHCFG2R1` writer"]
pub struct W(crate::W<DFSDM_CHCFG2R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_CHCFG2R1_SPEC>;
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
impl From<crate::W<DFSDM_CHCFG2R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_CHCFG2R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SITP` reader - Serial interface type for channel 2"]
pub type SITP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SITP` writer - Serial interface type for channel 2"]
pub type SITP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_CHCFG2R1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPICKSEL` reader - SPI clock select for channel 2"]
pub type SPICKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPICKSEL` writer - SPI clock select for channel 2"]
pub type SPICKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFSDM_CHCFG2R1_SPEC, u8, u8, 2, O>;
#[doc = "Field `SCDEN` reader - Short-circuit detector enable on channel 2"]
pub type SCDEN_R = crate::BitReader<bool>;
#[doc = "Field `SCDEN` writer - Short-circuit detector enable on channel 2"]
pub type SCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_CHCFG2R1_SPEC, bool, O>;
#[doc = "Field `CKABEN` reader - Clock absence detector enable on channel 2"]
pub type CKABEN_R = crate::BitReader<bool>;
#[doc = "Field `CKABEN` writer - Clock absence detector enable on channel 2"]
pub type CKABEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_CHCFG2R1_SPEC, bool, O>;
#[doc = "Field `CHEN` reader - Channel 2 enable"]
pub type CHEN_R = crate::BitReader<bool>;
#[doc = "Field `CHEN` writer - Channel 2 enable"]
pub type CHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_CHCFG2R1_SPEC, bool, O>;
#[doc = "Field `CHINSEL` reader - Channel inputs selection"]
pub type CHINSEL_R = crate::BitReader<bool>;
#[doc = "Field `CHINSEL` writer - Channel inputs selection"]
pub type CHINSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_CHCFG2R1_SPEC, bool, O>;
#[doc = "Field `DATMPX` reader - Input data multiplexer for channel 2"]
pub type DATMPX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATMPX` writer - Input data multiplexer for channel 2"]
pub type DATMPX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_CHCFG2R1_SPEC, u8, u8, 2, O>;
#[doc = "Field `DATPACK` reader - Data packing mode in DFSDM_CHDATINyR register"]
pub type DATPACK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATPACK` writer - Data packing mode in DFSDM_CHDATINyR register"]
pub type DATPACK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFSDM_CHCFG2R1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CKOUTDIV` reader - Output serial clock divider"]
pub type CKOUTDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKOUTDIV` writer - Output serial clock divider"]
pub type CKOUTDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFSDM_CHCFG2R1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CKOUTSRC` reader - Output serial clock source selection"]
pub type CKOUTSRC_R = crate::BitReader<bool>;
#[doc = "Field `CKOUTSRC` writer - Output serial clock source selection"]
pub type CKOUTSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_CHCFG2R1_SPEC, bool, O>;
#[doc = "Field `DFSDMEN` reader - Global enable for DFSDM interface"]
pub type DFSDMEN_R = crate::BitReader<bool>;
#[doc = "Field `DFSDMEN` writer - Global enable for DFSDM interface"]
pub type DFSDMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSDM_CHCFG2R1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Serial interface type for channel 2"]
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPI clock select for channel 2"]
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Short-circuit detector enable on channel 2"]
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock absence detector enable on channel 2"]
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 2 enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel inputs selection"]
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Input data multiplexer for channel 2"]
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Data packing mode in DFSDM_CHDATINyR register"]
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Output serial clock divider"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Output serial clock source selection"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Global enable for DFSDM interface"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Serial interface type for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn sitp(&mut self) -> SITP_W<0> {
        SITP_W::new(self)
    }
    #[doc = "Bits 2:3 - SPI clock select for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn spicksel(&mut self) -> SPICKSEL_W<2> {
        SPICKSEL_W::new(self)
    }
    #[doc = "Bit 5 - Short-circuit detector enable on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn scden(&mut self) -> SCDEN_W<5> {
        SCDEN_W::new(self)
    }
    #[doc = "Bit 6 - Clock absence detector enable on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn ckaben(&mut self) -> CKABEN_W<6> {
        CKABEN_W::new(self)
    }
    #[doc = "Bit 7 - Channel 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> CHEN_W<7> {
        CHEN_W::new(self)
    }
    #[doc = "Bit 8 - Channel inputs selection"]
    #[inline(always)]
    #[must_use]
    pub fn chinsel(&mut self) -> CHINSEL_W<8> {
        CHINSEL_W::new(self)
    }
    #[doc = "Bits 12:13 - Input data multiplexer for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn datmpx(&mut self) -> DATMPX_W<12> {
        DATMPX_W::new(self)
    }
    #[doc = "Bits 14:15 - Data packing mode in DFSDM_CHDATINyR register"]
    #[inline(always)]
    #[must_use]
    pub fn datpack(&mut self) -> DATPACK_W<14> {
        DATPACK_W::new(self)
    }
    #[doc = "Bits 16:23 - Output serial clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W<16> {
        CKOUTDIV_W::new(self)
    }
    #[doc = "Bit 30 - Output serial clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W<30> {
        CKOUTSRC_W::new(self)
    }
    #[doc = "Bit 31 - Global enable for DFSDM interface"]
    #[inline(always)]
    #[must_use]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W<31> {
        DFSDMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM channel configuration 2 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_chcfg2r1](index.html) module"]
pub struct DFSDM_CHCFG2R1_SPEC;
impl crate::RegisterSpec for DFSDM_CHCFG2R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_chcfg2r1::R](R) reader structure"]
impl crate::Readable for DFSDM_CHCFG2R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_chcfg2r1::W](W) writer structure"]
impl crate::Writable for DFSDM_CHCFG2R1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSDM_CHCFG2R1 to value 0"]
impl crate::Resettable for DFSDM_CHCFG2R1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
