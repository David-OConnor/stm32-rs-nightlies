#[doc = "Register `TIMx_BDTR` reader"]
pub struct R(crate::R<TIMX_BDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_BDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_BDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_BDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_BDTR` writer"]
pub struct W(crate::W<TIMX_BDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_BDTR_SPEC>;
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
impl From<crate::W<TIMX_BDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_BDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTG` reader - DTG"]
pub type DTG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTG` writer - DTG"]
pub type DTG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMX_BDTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMX_BDTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSSI` reader - OSSI"]
pub type OSSI_R = crate::BitReader<bool>;
#[doc = "Field `OSSI` writer - OSSI"]
pub type OSSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMX_BDTR_SPEC, bool, O>;
#[doc = "Field `OSSR` reader - OSSR"]
pub type OSSR_R = crate::BitReader<bool>;
#[doc = "Field `OSSR` writer - OSSR"]
pub type OSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMX_BDTR_SPEC, bool, O>;
#[doc = "Field `BKE` reader - BKE"]
pub type BKE_R = crate::BitReader<bool>;
#[doc = "Field `BKE` writer - BKE"]
pub type BKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMX_BDTR_SPEC, bool, O>;
#[doc = "Field `BKP` reader - BKP"]
pub type BKP_R = crate::BitReader<bool>;
#[doc = "Field `BKP` writer - BKP"]
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMX_BDTR_SPEC, bool, O>;
#[doc = "Field `AOE` reader - AOE"]
pub type AOE_R = crate::BitReader<bool>;
#[doc = "Field `AOE` writer - AOE"]
pub type AOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMX_BDTR_SPEC, bool, O>;
#[doc = "Field `MOE` reader - MOE"]
pub type MOE_R = crate::BitReader<bool>;
#[doc = "Field `MOE` writer - MOE"]
pub type MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMX_BDTR_SPEC, bool, O>;
#[doc = "Field `BKF` reader - BKF"]
pub type BKF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKF` writer - BKF"]
pub type BKF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMX_BDTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `BKDSRM` reader - BKDSRM"]
pub type BKDSRM_R = crate::BitReader<bool>;
#[doc = "Field `BKDSRM` writer - BKDSRM"]
pub type BKDSRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMX_BDTR_SPEC, bool, O>;
#[doc = "Field `BKBID` reader - BKBID"]
pub type BKBID_R = crate::BitReader<bool>;
#[doc = "Field `BKBID` writer - BKBID"]
pub type BKBID_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMX_BDTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - DTG"]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - OSSI"]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OSSR"]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BKE"]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AOE"]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MOE"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - BKF"]
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTG"]
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<0> {
        DTG_W::new(self)
    }
    #[doc = "Bits 8:9 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<8> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 10 - OSSI"]
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<10> {
        OSSI_W::new(self)
    }
    #[doc = "Bit 11 - OSSR"]
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<11> {
        OSSR_W::new(self)
    }
    #[doc = "Bit 12 - BKE"]
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<12> {
        BKE_W::new(self)
    }
    #[doc = "Bit 13 - BKP"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<13> {
        BKP_W::new(self)
    }
    #[doc = "Bit 14 - AOE"]
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<14> {
        AOE_W::new(self)
    }
    #[doc = "Bit 15 - MOE"]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<15> {
        MOE_W::new(self)
    }
    #[doc = "Bits 16:19 - BKF"]
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BKF_W<16> {
        BKF_W::new(self)
    }
    #[doc = "Bit 26 - BKDSRM"]
    #[inline(always)]
    #[must_use]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<26> {
        BKDSRM_W::new(self)
    }
    #[doc = "Bit 28 - BKBID"]
    #[inline(always)]
    #[must_use]
    pub fn bkbid(&mut self) -> BKBID_W<28> {
        BKBID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "As the BKBID, BKDSRM, BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
bits may be write-locked depending on the LOCK configuration, it may be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_bdtr](index.html) module"]
pub struct TIMX_BDTR_SPEC;
impl crate::RegisterSpec for TIMX_BDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_bdtr::R](R) reader structure"]
impl crate::Readable for TIMX_BDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_bdtr::W](W) writer structure"]
impl crate::Writable for TIMX_BDTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMx_BDTR to value 0"]
impl crate::Resettable for TIMX_BDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
