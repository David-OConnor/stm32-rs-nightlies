#[doc = "Register `CFGR2` reader"]
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR2` writer"]
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FWDIS` reader - Firewall disable bit"]
pub type FWDIS_R = crate::BitReader<bool>;
#[doc = "Field `FWDIS` writer - Firewall disable bit"]
pub type FWDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `I2C_PB6_FMP` reader - Fm+ drive capability on PB6 enable bit"]
pub type I2C_PB6_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB6_FMP` writer - Fm+ drive capability on PB6 enable bit"]
pub type I2C_PB6_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `I2C_PB7_FMP` reader - Fm+ drive capability on PB7 enable bit"]
pub type I2C_PB7_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB7_FMP` writer - Fm+ drive capability on PB7 enable bit"]
pub type I2C_PB7_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `I2C_PB8_FMP` reader - Fm+ drive capability on PB8 enable bit"]
pub type I2C_PB8_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB8_FMP` writer - Fm+ drive capability on PB8 enable bit"]
pub type I2C_PB8_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `I2C_PB9_FMP` reader - Fm+ drive capability on PB9 enable bit"]
pub type I2C_PB9_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C_PB9_FMP` writer - Fm+ drive capability on PB9 enable bit"]
pub type I2C_PB9_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `I2C1_FMP` reader - I2C1 Fm+ drive capability enable bit"]
pub type I2C1_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C1_FMP` writer - I2C1 Fm+ drive capability enable bit"]
pub type I2C1_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
#[doc = "Field `I2C2_FMP` reader - I2C2 Fm+ drive capability enable bit"]
pub type I2C2_FMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C2_FMP` writer - I2C2 Fm+ drive capability enable bit"]
pub type I2C2_FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Firewall disable bit"]
    #[inline(always)]
    pub fn fwdis(&self) -> FWDIS_R {
        FWDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Fm+ drive capability on PB6 enable bit"]
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fm+ drive capability on PB7 enable bit"]
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fm+ drive capability on PB8 enable bit"]
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fm+ drive capability on PB9 enable bit"]
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C1 Fm+ drive capability enable bit"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C2 Fm+ drive capability enable bit"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Firewall disable bit"]
    #[inline(always)]
    #[must_use]
    pub fn fwdis(&mut self) -> FWDIS_W<0> {
        FWDIS_W::new(self)
    }
    #[doc = "Bit 8 - Fm+ drive capability on PB6 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<8> {
        I2C_PB6_FMP_W::new(self)
    }
    #[doc = "Bit 9 - Fm+ drive capability on PB7 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<9> {
        I2C_PB7_FMP_W::new(self)
    }
    #[doc = "Bit 10 - Fm+ drive capability on PB8 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<10> {
        I2C_PB8_FMP_W::new(self)
    }
    #[doc = "Bit 11 - Fm+ drive capability on PB9 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<11> {
        I2C_PB9_FMP_W::new(self)
    }
    #[doc = "Bit 12 - I2C1 Fm+ drive capability enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<12> {
        I2C1_FMP_W::new(self)
    }
    #[doc = "Bit 13 - I2C2 Fm+ drive capability enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<13> {
        I2C2_FMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr2](index.html) module"]
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr2::R](R) reader structure"]
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr2::W](W) writer structure"]
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
