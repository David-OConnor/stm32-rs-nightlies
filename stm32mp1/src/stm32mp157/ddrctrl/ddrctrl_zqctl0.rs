#[doc = "Register `DDRCTRL_ZQCTL0` reader"]
pub struct R(crate::R<DDRCTRL_ZQCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ZQCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ZQCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ZQCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ZQCTL0` writer"]
pub struct W(crate::W<DDRCTRL_ZQCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ZQCTL0_SPEC>;
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
impl From<crate::W<DDRCTRL_ZQCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ZQCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_ZQ_SHORT_NOP` reader - T_ZQ_SHORT_NOP"]
pub type T_ZQ_SHORT_NOP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_ZQ_SHORT_NOP` writer - T_ZQ_SHORT_NOP"]
pub type T_ZQ_SHORT_NOP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ZQCTL0_SPEC, u16, u16, 10, O>;
#[doc = "Field `T_ZQ_LONG_NOP` reader - T_ZQ_LONG_NOP"]
pub type T_ZQ_LONG_NOP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_ZQ_LONG_NOP` writer - T_ZQ_LONG_NOP"]
pub type T_ZQ_LONG_NOP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ZQCTL0_SPEC, u16, u16, 11, O>;
#[doc = "Field `ZQ_RESISTOR_SHARED` reader - ZQ_RESISTOR_SHARED"]
pub type ZQ_RESISTOR_SHARED_R = crate::BitReader<bool>;
#[doc = "Field `ZQ_RESISTOR_SHARED` writer - ZQ_RESISTOR_SHARED"]
pub type ZQ_RESISTOR_SHARED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_ZQCTL0_SPEC, bool, O>;
#[doc = "Field `DIS_SRX_ZQCL` reader - DIS_SRX_ZQCL"]
pub type DIS_SRX_ZQCL_R = crate::BitReader<bool>;
#[doc = "Field `DIS_SRX_ZQCL` writer - DIS_SRX_ZQCL"]
pub type DIS_SRX_ZQCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_ZQCTL0_SPEC, bool, O>;
#[doc = "Field `DIS_AUTO_ZQ` reader - DIS_AUTO_ZQ"]
pub type DIS_AUTO_ZQ_R = crate::BitReader<bool>;
#[doc = "Field `DIS_AUTO_ZQ` writer - DIS_AUTO_ZQ"]
pub type DIS_AUTO_ZQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_ZQCTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - T_ZQ_SHORT_NOP"]
    #[inline(always)]
    pub fn t_zq_short_nop(&self) -> T_ZQ_SHORT_NOP_R {
        T_ZQ_SHORT_NOP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:26 - T_ZQ_LONG_NOP"]
    #[inline(always)]
    pub fn t_zq_long_nop(&self) -> T_ZQ_LONG_NOP_R {
        T_ZQ_LONG_NOP_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - ZQ_RESISTOR_SHARED"]
    #[inline(always)]
    pub fn zq_resistor_shared(&self) -> ZQ_RESISTOR_SHARED_R {
        ZQ_RESISTOR_SHARED_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DIS_SRX_ZQCL"]
    #[inline(always)]
    pub fn dis_srx_zqcl(&self) -> DIS_SRX_ZQCL_R {
        DIS_SRX_ZQCL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DIS_AUTO_ZQ"]
    #[inline(always)]
    pub fn dis_auto_zq(&self) -> DIS_AUTO_ZQ_R {
        DIS_AUTO_ZQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - T_ZQ_SHORT_NOP"]
    #[inline(always)]
    #[must_use]
    pub fn t_zq_short_nop(&mut self) -> T_ZQ_SHORT_NOP_W<0> {
        T_ZQ_SHORT_NOP_W::new(self)
    }
    #[doc = "Bits 16:26 - T_ZQ_LONG_NOP"]
    #[inline(always)]
    #[must_use]
    pub fn t_zq_long_nop(&mut self) -> T_ZQ_LONG_NOP_W<16> {
        T_ZQ_LONG_NOP_W::new(self)
    }
    #[doc = "Bit 29 - ZQ_RESISTOR_SHARED"]
    #[inline(always)]
    #[must_use]
    pub fn zq_resistor_shared(&mut self) -> ZQ_RESISTOR_SHARED_W<29> {
        ZQ_RESISTOR_SHARED_W::new(self)
    }
    #[doc = "Bit 30 - DIS_SRX_ZQCL"]
    #[inline(always)]
    #[must_use]
    pub fn dis_srx_zqcl(&mut self) -> DIS_SRX_ZQCL_W<30> {
        DIS_SRX_ZQCL_W::new(self)
    }
    #[doc = "Bit 31 - DIS_AUTO_ZQ"]
    #[inline(always)]
    #[must_use]
    pub fn dis_auto_zq(&mut self) -> DIS_AUTO_ZQ_W<31> {
        DIS_AUTO_ZQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL ZQ control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_zqctl0](index.html) module"]
pub struct DDRCTRL_ZQCTL0_SPEC;
impl crate::RegisterSpec for DDRCTRL_ZQCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_zqctl0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ZQCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_zqctl0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ZQCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ZQCTL0 to value 0x0200_0040"]
impl crate::Resettable for DDRCTRL_ZQCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0040;
}
